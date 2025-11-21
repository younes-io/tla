use std::path::PathBuf;
use tla_cli::lint::types::{RuleCode, Severity};
use tla_cli::lint::{collect_diagnostics, reporter};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from("fixtures").join(name)
}

#[test]
fn ok_fixture_has_no_diagnostics() {
    let diags = collect_diagnostics(vec![fixture("ok.tla")]).expect("lint run");
    assert!(diags.is_empty());
}

#[test]
fn unused_variable_warns() {
    let diags = collect_diagnostics(vec![fixture("unused.tla")]).expect("lint run");
    assert_eq!(diags.len(), 1);
    let d = &diags[0];
    assert_eq!(d.code, RuleCode::TLA001);
    assert_eq!(d.severity, Severity::Warning);
}

#[test]
fn missing_next_errors() {
    let diags = collect_diagnostics(vec![fixture("missing_next.tla")]).expect("lint run");
    assert!(
        diags
            .iter()
            .any(|d| d.code == RuleCode::TLA002 && d.severity == Severity::Error)
    );
}

#[test]
fn json_output_is_stable() {
    let diags = collect_diagnostics(vec![fixture("unused.tla"), fixture("missing_next.tla")])
        .expect("lint run");
    let json = reporter::to_json(&diags).expect("json");
    insta::assert_snapshot!("json_output", json);
}

#[test]
fn ok_fixture_tree_shape() {
    use tla_cli::tla_parser::TlaParser;
    let mut parser = TlaParser::new().unwrap();
    let src = std::fs::read_to_string(fixture("ok.tla")).unwrap();
    let tree = parser.parse(&src).expect("parse ok");
    let root = tree.root_node();
    assert!(!root.has_error());

    let mut pairs = Vec::new();
    fn walk(node: tree_sitter::Node, src: &str, out: &mut Vec<(String, String)>) {
        if node.kind() == "identifier" || node.kind() == "identifier_ref" {
            let txt = node.utf8_text(src.as_bytes()).unwrap().to_string();
            let parent = node
                .parent()
                .map(|p| p.kind().to_string())
                .unwrap_or_default();
            out.push((txt, parent));
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk(child, src, out);
        }
    }

    walk(root, &src, &mut pairs);

    assert_eq!(
        pairs,
        vec![
            ("Ok".into(), "module".into()),
            ("x".into(), "variable_declaration".into()),
            ("Init".into(), "operator_definition".into()),
            ("x".into(), "bound_infix_op".into()), // occurrence in Init definition
            ("Next".into(), "operator_definition".into()),
            ("x".into(), "bound_postfix_op".into()), // x' occurrence
            ("x".into(), "bound_infix_op".into()),   // rhs occurrence
        ]
    );
}
