pub fn is_excluded(path: &str) -> bool {
    let p = path.to_ascii_lowercase();

    p.contains("$recycle.bin")
        || p.contains("system volume information")
        || p.starts_with("c:\\windows")
        || p.starts_with("c:\\program files")
}
