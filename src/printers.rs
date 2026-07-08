use std::path::Path;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<String> = OnceLock::new();

fn program_name() -> &'static str {
    PROGRAM_NAME.get_or_init(|| {
        std::env::current_exe()
            .ok()
            .and_then(|pb| Path::new(&pb).file_name().map(|s| s.to_os_string()))
            .and_then(|s| s.into_string().ok())
            .unwrap_or_else(|| "gpull".to_string())
    })
}

// Wrap println! in order to prefix the program name
pub fn sdout(args: std::fmt::Arguments) {
    println!("[{}]: {}", program_name(), args);
}

// Wrap eprintln! in order to prefix the program name
pub fn sderr(args: std::fmt::Arguments) {
    eprintln!("[{}]: {}", program_name(), args);
}
