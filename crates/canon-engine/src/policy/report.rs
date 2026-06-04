use std::collections::BTreeMap;

/// Groups a list of violations (file_path, message) by their top-level module or directory.
pub fn group_violations_by_module<'a>(
    violations: &'a [(&'a str, &'a str)],
) -> BTreeMap<String, Vec<&'a str>> {
    let mut grouped: BTreeMap<String, Vec<&'a str>> = BTreeMap::new();
    for (path, message) in violations {
        let parts: Vec<&str> = path.split('/').collect();
        let module = if parts.len() > 1 && parts[0] == "crates" {
            format!("{}/{}", parts[0], parts[1])
        } else if !parts.is_empty() {
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
