use canon_engine::policy::report::{group_violations_by_module, paginate_report};

#[test]
fn test_group_violations_by_module() {
    let violations = vec![
        ("crates/canon-engine/src/domain.rs", "Missing docstring"),
        ("crates/canon-engine/src/domain.rs", "Use of panic"),
        ("crates/canon-cli/src/main.rs", "Unwrap used"),
    ];

    let grouped = group_violations_by_module(&violations);
    assert_eq!(grouped.len(), 2);
    assert_eq!(grouped.get("crates/canon-engine").unwrap().len(), 2);
    assert_eq!(grouped.get("crates/canon-cli").unwrap().len(), 1);
}

#[test]
fn test_paginate_report() {
    let mut items = Vec::new();
    for i in 0..105 {
        items.push(format!("Violation {}", i));
    }

    let pages = paginate_report(&items, 50);
    assert_eq!(pages.len(), 3);
    assert_eq!(pages[0].len(), 50);
    assert_eq!(pages[1].len(), 50);
    assert_eq!(pages[2].len(), 5);
}
