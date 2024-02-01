use lazy_static::lazy_static;
use std::path::Path;
use std::sync::Mutex;

// Lazily initialize the running program's name the first time it's accessed
lazy_static! {
    static ref PROGRAM_NAME: Mutex<String> = Mutex::new(
        std::env::current_exe()
            .ok()
            .and_then(|pb| Path::new(&pb).file_name().map(|s| s.to_os_string()))
            .and_then(|s| s.into_string().ok())
            .unwrap_or_else(|| "gpull".to_string())
    );
}

// Wrap println! in order to prefix the program name
pub fn sdout(args: std::fmt::Arguments) {
    let program_name = PROGRAM_NAME.lock().unwrap();
    println!("[{}]: {}", *program_name, args);
}

// Wrap eprintln! in order to prefix the program name
pub fn sderr(args: std::fmt::Arguments) {
    let program_name = PROGRAM_NAME.lock().unwrap();
    eprintln!("[{}]: {}", *program_name, args);
}
