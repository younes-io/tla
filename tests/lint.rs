use std::path::PathBuf;
use tla::lint::types::{RuleCode, Severity};
use tla::lint::{collect_diagnostics, reporter};

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
#[ignore]
fn debug_identifiers_in_ok() {
    use tla::tla_parser::TlaParser;
    let mut parser = TlaParser::new().unwrap();
    let src = std::fs::read_to_string(fixture("ok.tla")).unwrap();
    let tree = parser.parse(&src).unwrap();
    let root = tree.root_node();
    eprintln!("{}", root.to_sexp());

    fn walk(node: tree_sitter::Node, src: &str) {
        if node.kind() == "identifier" {
            let txt = node.utf8_text(src.as_bytes()).unwrap();
            let parent = node
                .parent()
                .map(|p| p.kind().to_string())
                .unwrap_or_default();
            eprintln!("identifier `{}` parent {}", txt, parent);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            walk(child, src);
        }
    }

    walk(root, &src);
}
