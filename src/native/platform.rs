mod platform_lib {
    use std::env;

    pub fn shell_cmd() -> (&'static str, &'static str) {
        if cfg!(target_os = "windows") {
            ("cmd.exe", "/C")
        } else {
            ("sh", "-c")
        }
    }

    pub fn which_cmd() -> &'static str {
        if cfg!(target_os = "windows") {
            "where"
        } else {
            "which"
        }
    }

    pub fn output_extension() -> &'static str {
        if cfg!(target_os = "windows") {
            ".exe"
        } else {
            ""
        }
    }

    pub fn target_os() -> &'static str {
        env::consts::OS
    }

    pub fn target_arch() -> &'static str {
        env::consts::ARCH
    }

    pub fn is_windows() -> bool {
        cfg!(target_os = "windows")
    }

    pub fn is_unix() -> bool {
        cfg!(unix)
    }

    pub fn is_linux() -> bool {
        cfg!(target_os = "linux")
    }

    pub fn is_macos() -> bool {
        cfg!(target_os = "macos")
    }

    pub fn default_cc() -> &'static str {
        if is_windows() {
            "gcc"
        } else if is_macos() {
            "clang"
        } else {
            "gcc"
        }
    }
}

pub use platform_lib::*;
