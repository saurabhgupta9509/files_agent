use crate::rules::rule::Rule;
use crate::rules::operation::Operation;
use std::path::Path;
use std::path::PathBuf;

/// Try to find the first matching rule
pub fn match_rule<'a>(
    event_path: &str,
    event_operation: &Operation,
    rules: &'a [Rule],
) -> Option<&'a Rule> {

    let normalized_event_path = normalize_path(event_path);

    // Sort rules by path length (longest first → more specific wins)
    let mut sorted_rules: Vec<&Rule> = rules
        .iter()
        .filter(|r| r.enabled)
        .collect();

    sorted_rules.sort_by_key(|r| std::cmp::Reverse(r.path.len()));

    for rule in sorted_rules {
        if !rule.operations.contains(event_operation) {
            continue;
        }

        if path_matches(&normalized_event_path, &rule.path, rule.recursive) {
            return Some(rule);
        }
    }

    None
}

fn path_matches(event_path: &str, rule_path: &str, recursive: bool) -> bool {
    let normalized_rule_path = normalize_path(rule_path);

    if recursive {
        event_path.starts_with(&normalized_rule_path)
    } else {
        event_path == normalized_rule_path
    }
}
fn normalize_path(path: &str) -> String {
    Path::new(path)
        .components()
        .collect::<PathBuf>()
        .to_string_lossy()
        .to_lowercase()
}

