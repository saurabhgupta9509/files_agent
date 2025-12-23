use std::path::Path;

#[derive(Debug)]
pub enum RuleTarget {
    File,
    Directory,
}

pub fn detect_rule_target(path: &str) -> Option<RuleTarget> {
    let p = Path::new(path);

    // If path ends with separator → directory intent
    if path.ends_with('\\') || path.ends_with('/') {
        return Some(RuleTarget::Directory);
    }

    // If path has an extension → file intent
    if p.extension().is_some() {
        return Some(RuleTarget::File);
    }

    // Otherwise treat as directory by default
    Some(RuleTarget::Directory)
}
