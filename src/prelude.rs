// Global types
pub type Result<T> = core::result::Result<T, Error>;

/// Executes given command using `sh`
#[macro_export]
macro_rules! execute_cmd {
    ($cmd:expr) => {
        let out = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("{}", $cmd))
            .output()
            .unwrap_or_else(|_| panic!("Command \"{}\" failed!", $cmd));
        dbg!(out);
    };
}

/// Re-exports
pub use crate::error::Error;
