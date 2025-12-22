use std::collections::HashMap;
use std::time::{Instant, Duration};

struct Activity {
    count: u32,
    first_seen: Instant,
}

static mut ACTIVITY_MAP: Option<HashMap<String, Activity>> = None;

pub fn detect_mass_delete(folder: &str) -> bool {
    unsafe {
        let map = ACTIVITY_MAP.get_or_insert(HashMap::new());
        let entry = map.entry(folder.to_string())
            .or_insert(Activity {
                count: 0,
                first_seen: Instant::now(),
            });

        entry.count += 1;

        entry.count > 20 &&
        entry.first_seen.elapsed() < Duration::from_secs(5)
    }
}
