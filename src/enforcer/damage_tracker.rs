use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DAMAGE_MAP: Mutex<HashMap<String, (u32, Instant)>> =
        Mutex::new(HashMap::new());
}

pub fn record_damage(path: &str) -> bool {
    let mut map = DAMAGE_MAP.lock().unwrap();
    let entry = map.entry(path.to_string())
        .or_insert((0, Instant::now()));

    entry.0 += 1;

    if entry.0 > 10 && entry.1.elapsed() < Duration::from_secs(10) {
        return true; // escalation needed
    }

    false
}
