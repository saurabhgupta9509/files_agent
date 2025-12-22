use crate::models::fs_event::FileSystemEvent;


pub async fn send_event(event: FileSystemEvent) {
    println!("EVENT: {:?}", event);
    // TEMP: print only
    // Later: HTTP POST to server
}
