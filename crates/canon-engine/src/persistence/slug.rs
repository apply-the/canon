//! Slug derivation and sanitization for run directory names.
//!
//! See `specs/009-run-id-display/contracts/run-identity-contract.md` §C-3.
//! Slug is descriptive metadata only and is never required for identity
//! or uniqueness; an empty result returns `None`.

const MAX_SLUG_LEN: usize = 60;

/// Sanitize an arbitrary source string into an optional slug.
///
/// Pipeline:
///
/// 1. Lowercase the input.
/// 2. ASCII-fold; non-ASCII characters that have no fold are dropped.
/// 3. Replace any run of characters not in `[a-z0-9]` with a single `-`.
/// 4. Trim leading and trailing `-`.
/// 5. Truncate to at most 60 characters; trim a trailing `-` left after
///    truncation.
/// 6. If the result is empty, return `None`.
pub fn slugify(source: &str) -> Option<String> {
    let mut buf = String::with_capacity(source.len());
    let mut last_was_sep = true;
    for ch in source.chars() {
        let lower = ch.to_ascii_lowercase();
        let mapped = if lower.is_ascii_alphanumeric() {
            Some(lower)
        } else if lower.is_ascii() {
            None
        } else {
            // Drop non-ASCII characters that have no simple fold.
            None
        };
        match mapped {
            Some(c) => {
                buf.push(c);
                last_was_sep = false;
            }
            None => {
                if !last_was_sep {
                    buf.push('-');
                    last_was_sep = true;
                }
            }
        }
    }

    // Trim leading/trailing '-'
    let trimmed = buf.trim_matches('-');
    if trimmed.is_empty() {
        return None;
    }

    // Cap length and re-trim trailing '-' if truncation left one.
    let mut out: String = trimmed.chars().take(MAX_SLUG_LEN).collect();
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() { None } else { Some(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_yields_none() {
        assert_eq!(slugify(""), None);
        assert_eq!(slugify("   "), None);
    }

    #[test]
    fn pure_punctuation_yields_none() {
        assert_eq!(slugify("***"), None);
        assert_eq!(slugify("!!!---???"), None);
    }

    #[test]
    fn lowercases_and_replaces_separators() {
        assert_eq!(slugify("Auth Hardening Scope").as_deref(), Some("auth-hardening-scope"));
    }

    #[test]
    fn collapses_repeated_separators() {
        assert_eq!(slugify("foo___bar---baz").as_deref(), Some("foo-bar-baz"));
        assert_eq!(slugify("a   b   c").as_deref(), Some("a-b-c"));
    }

    #[test]
    fn trims_leading_and_trailing_separators() {
        assert_eq!(slugify("---hello---").as_deref(), Some("hello"));
        assert_eq!(slugify("  hello  ").as_deref(), Some("hello"));
    }

    #[test]
    fn drops_non_ascii() {
        assert_eq!(slugify("café résumé").as_deref(), Some("caf-r-sum"));
        assert_eq!(slugify("日本語タイトル"), None);
    }

    #[test]
    fn caps_length_at_60_and_trims_trailing_dash() {
        let long = "a".repeat(70);
        let result = slugify(&long).expect("non-empty");
        assert_eq!(result.len(), 60);
        assert!(result.chars().all(|c| c == 'a'));

        let with_dash = format!("{}-{}", "a".repeat(58), "garbage that gets cut");
        let result = slugify(&with_dash).expect("non-empty");
        assert!(result.len() <= 60);
        assert!(!result.ends_with('-'));
    }

    #[test]
    fn matches_contract_regex() {
        let re = |s: &str| {
            !s.is_empty()
                && s.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '-'))
                && !s.starts_with('-')
                && !s.ends_with('-')
                && s.len() <= 60
        };
        for src in [
            "Hello World",
            "a/b/c",
            "FOO_BAR_baz_123",
            "  many   spaces here  ",
            "----dashes-everywhere----",
        ] {
            let slug = slugify(src).unwrap_or_default();
            assert!(re(&slug), "slug {slug:?} from {src:?} fails regex");
        }
    }
}
