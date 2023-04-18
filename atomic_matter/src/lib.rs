pub mod atoms;
pub mod dependencies;
pub mod executor;
pub mod inputs;
use anyhow::{anyhow, Result};
use std::env::consts::OS;
/// Returns if the current process is elevated within a Windows OS
// https://doc.rust-lang.org/reference/conditional-compilation.html
#[cfg(target_os = "windows")]
pub fn is_elevated() -> bool {
    extern crate winapi;
    use std::mem;
    use winapi::shared::minwindef::DWORD;
    use winapi::shared::minwindef::LPVOID;
    use winapi::um::processthreadsapi::GetCurrentProcess;
    use winapi::um::processthreadsapi::OpenProcessToken;
    use winapi::um::securitybaseapi::GetTokenInformation;
    use winapi::um::winnt::TokenElevation;
    use winapi::um::winnt::HANDLE;
    use winapi::um::winnt::TOKEN_ELEVATION;
    use winapi::um::winnt::TOKEN_QUERY;
    // here is an implementation to use https://github.com/microsoft/windows-rs/blob/0.48.0/crates/samples/windows/privileges/src/main.rs
    // https://docs.rs/is_elevated/latest/src/is_elevated/lib.rs.html#45-69
    // based on https://stackoverflow.com/a/8196291
    unsafe {
        let mut current_token_ptr: HANDLE = mem::zeroed();
        let mut token_elevation: TOKEN_ELEVATION = mem::zeroed();
        let token_elevation_type_ptr: *mut TOKEN_ELEVATION = &mut token_elevation;
        let mut size: DWORD = 0;

        let result = OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut current_token_ptr);

        if result != 0 {
            let result = GetTokenInformation(
                current_token_ptr,
                TokenElevation,
                token_elevation_type_ptr as LPVOID,
                mem::size_of::<winapi::um::winnt::TOKEN_ELEVATION_TYPE>() as u32,
                &mut size,
            );
            if result != 0 {
                return token_elevation.TokenIsElevated != 0;
            }
        }
    }
    false
}

/// Return if current process is elevated within linux
#[cfg(target_os = "linux")]
pub fn is_elevated() -> bool {
    use libc::geteuid;

    unsafe {
        if geteuid() == 0 {
            return true;
        }
    }

    false
}

/// Return if current process is elevated within linux
#[cfg(target_os = "macos")]
pub fn is_elevated() -> bool {
    use libc::geteuid;

    unsafe {
        if geteuid() == 0 {
            return true;
        }
    }

    false
}

/// Enumeration for given executors used to execute AtomicTest tests
pub enum Executors {
    Powershell,
    CommandPrompt,
    Sh,
    Bash,
    Manual,
}

impl Executors {
    /// Converts a string to respective enumeration
    pub fn convert(executor: &str) -> Result<Executors> {
        match executor.to_lowercase().as_str() {
            "powershell" => Ok(Executors::Powershell),
            "pwsh" => Ok(Executors::Powershell),
            "bash" => Ok(Executors::Bash),
            "sh" => Ok(Executors::Sh),
            "cmd" => Ok(Executors::CommandPrompt),
            "manual" => Ok(Executors::Manual),
            _ => Err(anyhow!(
                "Invalid executor provided, failed to find match for: {}",
                executor
            )),
        }
    }

    /// Returns the string value of the enumeration
    pub fn value(&self) -> &str {
        match *self {
            Executors::Powershell => "powershell",
            Executors::Bash => "bash",
            Executors::Sh => "sh",
            Executors::Manual => "manual",
            Executors::CommandPrompt => "cmd",
        }
    }
}

/// Returns the respective tuple command to be executed when using windows vs linux/macos
pub fn get_cmd_setup(executor: &str) -> (&str, &str) {
    if OS == "windows" {
        match executor.trim() {
            "powershell" => ("powershell.exe", "-c"),
            "command_prompt" => ("cmd.exe", "/c"),
            _ => panic!("invalid executor for windows specified"),
        }
    } else if OS == "macos" || OS == "linux" {
        match executor.trim() {
            "powershell" => ("pwsh", "-c"),
            "bash" => ("bash", "-c"),
            "sh" => ("sh", "-c"),
            _ => panic!("invalid executor for macos specified"),
        }
    } else {
        panic!("bad executor specified")
    }
}
