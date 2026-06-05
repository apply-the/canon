use std::collections::BTreeMap;

/// Groups a list of violations (file_path, message) by their top-level module or directory.
pub fn group_violations_by_module<'a>(
    violations: &'a [(&'a str, &'a str)],
) -> BTreeMap<String, Vec<&'a str>> {
    let mut grouped: BTreeMap<String, Vec<&'a str>> = BTreeMap::new();
    for (path, message) in violations {
        let parts: Vec<&str> = path.split('/').collect();
        let module = if path.is_empty() {
            "unknown".to_string()
        } else if parts.len() > 1 && parts[0] == "crates" {
            format!("{}/{}", parts[0], parts[1])
        } else if !parts.is_empty() && !parts[0].is_empty() {
            parts[0].to_string()
        } else {
            "unknown".to_string()
        };

        grouped.entry(module).or_default().push(*message);
    }
    grouped
}

/// Paginates a list of string items into chunks of the given page size.
pub fn paginate_report(items: &[String], page_size: usize) -> Vec<Vec<String>> {
    let mut pages = Vec::new();
    let mut current_page = Vec::new();

    for item in items {
        if current_page.len() >= page_size {
            pages.push(current_page);
            current_page = Vec::new();
        }
        current_page.push(item.clone());
    }

    if !current_page.is_empty() {
        pages.push(current_page);
    }

    pages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_violations() {
        let violations = vec![
            ("crates/canon-engine/src/file.rs", "error 1"),
            ("crates/canon-engine/src/other.rs", "error 2"),
            ("crates/canon-cli/src/main.rs", "error 3"),
            ("other/path.rs", "error 4"),
            ("", "error 5"),
            ("/foo/bar", "error 6"),
        ];
        let grouped = group_violations_by_module(&violations);
        assert_eq!(grouped.get("crates/canon-engine").unwrap().len(), 2);
        assert_eq!(grouped.get("crates/canon-cli").unwrap().len(), 1);
        assert_eq!(grouped.get("other").unwrap().len(), 1);
        assert_eq!(grouped.get("unknown").unwrap().len(), 2);
    }

    #[test]
    fn test_paginate_report() {
        let items: Vec<String> =
            vec!["1", "2", "3", "4", "5"].into_iter().map(String::from).collect();
        let pages = paginate_report(&items, 2);
        assert_eq!(pages.len(), 3);
        assert_eq!(pages[0].len(), 2);
        assert_eq!(pages[1].len(), 2);
        assert_eq!(pages[2].len(), 1);
    }
}
