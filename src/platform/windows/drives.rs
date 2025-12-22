use windows::Win32::Storage::FileSystem::GetLogicalDrives;

pub fn get_all_drives() -> Vec<String> {
    let mut drives = Vec::new();

    unsafe {
        let mask = GetLogicalDrives();

        for i in 0..26 {
            if (mask & (1 << i)) != 0 {
                let letter = (b'A' + i as u8) as char;
                drives.push(format!("{}:\\", letter));
            }
        }
    }

    drives
}
