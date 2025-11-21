use crate::lint::FileContext;
use crate::lint::types::{Diagnostic, RuleCode, Severity};
use std::collections::{HashMap, HashSet};
use tree_sitter::{Node, Tree};

pub fn run_all_rules(ctx: &FileContext, tree: &Tree, diags: &mut Vec<Diagnostic>) {
    rule_unused_variables(ctx, tree, diags);
    rule_missing_init_next(ctx, tree, diags);
}

fn rule_unused_variables(ctx: &FileContext, tree: &Tree, diags: &mut Vec<Diagnostic>) {
    let root = tree.root_node();
    let mut declared: HashMap<String, usize> = HashMap::new();
    traverse(root, &mut |node| {
        if is_decl_node(node) {
            for ident in child_identifiers(node) {
                if let Ok(text) = ident.utf8_text(ctx.src.as_bytes()) {
                    declared.insert(text.to_string(), ident.start_byte());
                }
            }
        }
    });

    let mut used: HashSet<String> = HashSet::new();
    traverse(root, &mut |node| {
        if is_identifier_use(node) {
            if let Some(parent) = node.parent() {
                if is_decl_node(parent) {
                    return;
                }
            }
            if let Ok(text) = node.utf8_text(ctx.src.as_bytes()) {
                used.insert(text.to_string());
            }
        }
    });

    for (name, byte_offset) in declared {
        if !used.contains(&name) {
            let (line, col) = ctx.position(byte_offset);
            diags.push(Diagnostic {
                file: ctx.path.to_string_lossy().into_owned(),
                line,
                column: col,
                severity: Severity::Warning,
                code: RuleCode::TLA001,
                message: format!("Variable `{}` is declared but never used", name),
            });
        }
    }
}

fn rule_missing_init_next(ctx: &FileContext, tree: &Tree, diags: &mut Vec<Diagnostic>) {
    let root = tree.root_node();
    let mut has_variables = false;
    let mut has_init = false;
    let mut has_next = false;

    traverse(root, &mut |node| {
        if is_decl_node(node) {
            has_variables = true;
        }

        if node.kind() == "operator_definition" {
            if let Some(name_node) = node
                .children(&mut node.walk())
                .find(|n| n.kind() == "identifier")
            {
                if let Ok(text) = name_node.utf8_text(ctx.src.as_bytes()) {
                    match text {
                        "Init" => has_init = true,
                        "Next" => has_next = true,
                        _ => {}
                    }
                }
            }
        }
    });

    if !has_variables {
        return;
    }

    if !has_init {
        diags.push(Diagnostic {
            file: ctx.path.to_string_lossy().into_owned(),
            line: 1,
            column: 1,
            severity: Severity::Error,
            code: RuleCode::TLA002,
            message: "Module declares VARIABLES but is missing Init operator".to_string(),
        });
    }
    if !has_next {
        diags.push(Diagnostic {
            file: ctx.path.to_string_lossy().into_owned(),
            line: 1,
            column: 1,
            severity: Severity::Error,
            code: RuleCode::TLA002,
            message: "Module declares VARIABLES but is missing Next operator".to_string(),
        });
    }
}

fn child_identifiers(node: Node) -> Vec<Node> {
    let mut idents = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "identifier" {
            idents.push(child);
        }
    }
    idents
}

fn is_decl_node(node: Node) -> bool {
    node.kind() == "variable_declaration" || node.kind() == "constant_declaration"
}

fn is_identifier_use(node: Node) -> bool {
    matches!(node.kind(), "identifier" | "identifier_ref")
}

fn traverse<F>(node: Node, f: &mut F)
where
    F: FnMut(Node),
{
    f(node);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        traverse(child, f);
    }
}
