#[cfg(all(windows, feature = "gui"))]
fn main() {
    use std::path::Path;
    
    let mut res = winres::WindowsResource::new();
    res.set("ProductName", "Telegram WS Proxy")
        .set("FileDescription", "Telegram MTProto WebSocket Bridge Proxy")
        .set("LegalCopyright", "MIT License");
    
    // Only set icon if it exists
    if Path::new("icon.ico").exists() {
        res.set_icon("icon.ico");
    }
    
    // Ignore error if compilation fails
    let _ = res.compile();
}

#[cfg(not(all(windows, feature = "gui")))]
fn main() {
    // Nothing to do on non-Windows platforms or without GUI
}
