use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X86_64,
    AArch64,
    ARM,
    RISCV64,
    I386,
}

impl Arch {
    pub fn from_host() -> Self {
        #[cfg(target_arch = "x86_64")]
        return Arch::X86_64;
        #[cfg(target_arch = "aarch64")]
        return Arch::AArch64;
        #[cfg(target_arch = "arm")]
        return Arch::ARM;
        #[cfg(target_arch = "riscv64")]
        return Arch::RISCV64;
        #[cfg(target_arch = "x86")]
        return Arch::I386;
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm", target_arch = "riscv64", target_arch = "x86")))]
        return Arch::X86_64;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OS {
    Linux,
    Windows,
    MacOS,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Android,
    IOS,
}

impl OS {
    pub fn from_host() -> Self {
        #[cfg(target_os = "linux")]
        return OS::Linux;
        #[cfg(target_os = "windows")]
        return OS::Windows;
        #[cfg(target_os = "macos")]
        return OS::MacOS;
        #[cfg(target_os = "freebsd")]
        return OS::FreeBSD;
        #[cfg(target_os = "openbsd")]
        return OS::OpenBSD;
        #[cfg(target_os = "netbsd")]
        return OS::NetBSD;
        #[cfg(target_os = "android")]
        return OS::Android;
        #[cfg(target_os = "ios")]
        return OS::IOS;
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd", target_os = "android", target_os = "ios")))]
        return OS::Linux;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ABI {
    SystemV,
    MSVC,
    Darwin,
    Android,
}

impl ABI {
    pub fn default_for(arch: Arch, os: OS) -> Self {
        match (arch, os) {
            (Arch::X86_64, OS::Windows) => ABI::MSVC,
            (_, OS::MacOS) => ABI::Darwin,
            (_, OS::Android) => ABI::Android,
            _ => ABI::SystemV,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TargetTriple {
    pub arch: Arch,
    pub vendor: String,
    pub os: OS,
    pub abi: ABI,
}

impl TargetTriple {
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() < 3 {
            return TargetTriple::host();
        }
        
        let arch = match parts[0].to_lowercase().as_str() {
            "x86_64" | "x64" | "amd64" => Arch::X86_64,
            "aarch64" | "arm64" => Arch::AArch64,
            "arm" | "armv7" | "armv7a" => Arch::ARM,
            "riscv64" => Arch::RISCV64,
            "i386" | "i686" | "x86" => Arch::I386,
            _ => Arch::X86_64,
        };
        
        let vendor = parts[1].to_string();
        let os = match parts[2].to_lowercase().as_str() {
            "linux" | "linux-gnu" | "linux-musl" => OS::Linux,
            "windows" | "windows-msvc" | "windows-gnu" | "cygwin" | "mingw" => OS::Windows,
            "darwin" | "macos" | "ios" => OS::MacOS,
            "freebsd" => OS::FreeBSD,
            "openbsd" => OS::OpenBSD,
            "netbsd" => OS::NetBSD,
            "android" => OS::Android,
            "ios" => OS::IOS,
            _ => OS::Linux,
        };
        
        let abi = if parts.len() > 3 {
            match parts[3].to_lowercase().as_str() {
                "msvc" => ABI::MSVC,
                "darwin" => ABI::Darwin,
                "android" | "androideabi" => ABI::Android,
                _ => ABI::SystemV,
            }
        } else {
            ABI::default_for(arch, os)
        };
        
        TargetTriple { arch, vendor, os, abi }
    }
    
    pub fn host() -> Self {
        TargetTriple {
            arch: Arch::from_host(),
            vendor: "unknown".to_string(),
            os: OS::from_host(),
            abi: ABI::SystemV,
        }
    }
    
    pub fn as_string(&self) -> String {
        let arch_str = match self.arch {
            Arch::X86_64 => "x86_64",
            Arch::AArch64 => "aarch64",
            Arch::ARM => "arm",
            Arch::RISCV64 => "riscv64",
            Arch::I386 => "i686",
        };
        
        let os_str = match self.os {
            OS::Linux => "linux",
            OS::Windows => "windows",
            OS::MacOS => "darwin",
            OS::FreeBSD => "freebsd",
            OS::OpenBSD => "openbsd",
            OS::NetBSD => "netbsd",
            OS::Android => "android",
            OS::IOS => "ios",
        };
        
        let abi_str = match self.abi {
            ABI::SystemV => "gnu",
            ABI::MSVC => "msvc",
            ABI::Darwin => "darwin",
            ABI::Android => "android",
        };
        
        format!("{}-{}-{}-{}", arch_str, self.vendor, os_str, abi_str)
    }
    
    pub fn gcc_target(&self) -> String {
        let arch_str = match self.arch {
            Arch::X86_64 => "x86_64",
            Arch::AArch64 => "aarch64",
            Arch::ARM => "arm",
            Arch::RISCV64 => "riscv64",
            Arch::I386 => "i686",
        };
        
        match self.os {
            OS::Linux => format!("{}-linux-gnu", arch_str),
            OS::Windows => format!("{}-pc-windows-msvc", arch_str),
            OS::MacOS => format!("{}-apple-darwin", arch_str),
            OS::Android => format!("{}-linux-android", arch_str),
            _ => format!("{}-{}-{}", arch_str, self.vendor, self.os_str()),
        }
    }
    
    fn os_str(&self) -> &'static str {
        match self.os {
            OS::Linux => "linux",
            OS::Windows => "windows",
            OS::MacOS => "macos",
            OS::FreeBSD => "freebsd",
            OS::OpenBSD => "openbsd",
            OS::NetBSD => "netbsd",
            OS::Android => "android",
            OS::IOS => "ios",
        }
    }
    
    pub fn linker_flags(&self) -> Vec<String> {
        let mut flags = Vec::new();
        
        match self.os {
            OS::Windows => {
                flags.push("-fuse-ld=lld".to_string());
                if self.arch == Arch::X86_64 {
                    flags.push("-m64".to_string());
                }
            }
            OS::MacOS => {
                flags.push("-target".to_string());
                flags.push(self.gcc_target());
            }
            OS::Android => {
                flags.push("--sysroot".to_string());
                flags.push("$ANDROID_NDK/toolchains/llvm/prebuilt/linux-x86_64/sysroot".to_string());
            }
            _ => {}
        }
        
        flags
    }
    
    pub fn cc_command(&self) -> String {
        match self.os {
            OS::Windows => "clang".to_string(),
            _ => "gcc".to_string(),
        }
    }
    
    pub fn supports_feature(&self, feature: &str) -> bool {
        match feature {
            "sse2" | "avx" | "avx2" => self.arch == Arch::X86_64,
            "neon" => self.arch == Arch::AArch64 || self.arch == Arch::ARM,
            "simd" => matches!(self.arch, Arch::X86_64 | Arch::AArch64),
            "atomic" => !matches!(self.arch, Arch::ARM | Arch::I386),
            _ => true,
        }
    }
}

pub fn list_supported_targets() -> Vec<TargetTriple> {
    vec![
        TargetTriple { arch: Arch::X86_64, vendor: "unknown".to_string(), os: OS::Linux, abi: ABI::SystemV },
        TargetTriple { arch: Arch::X86_64, vendor: "pc".to_string(), os: OS::Windows, abi: ABI::MSVC },
        TargetTriple { arch: Arch::X86_64, vendor: "apple".to_string(), os: OS::MacOS, abi: ABI::Darwin },
        TargetTriple { arch: Arch::AArch64, vendor: "unknown".to_string(), os: OS::Linux, abi: ABI::SystemV },
        TargetTriple { arch: Arch::AArch64, vendor: "pc".to_string(), os: OS::Windows, abi: ABI::MSVC },
        TargetTriple { arch: Arch::AArch64, vendor: "apple".to_string(), os: OS::MacOS, abi: ABI::Darwin },
        TargetTriple { arch: Arch::ARM, vendor: "unknown".to_string(), os: OS::Linux, abi: ABI::SystemV },
        TargetTriple { arch: Arch::RISCV64, vendor: "unknown".to_string(), os: OS::Linux, abi: ABI::SystemV },
        TargetTriple { arch: Arch::I386, vendor: "pc".to_string(), os: OS::Linux, abi: ABI::SystemV },
        TargetTriple { arch: Arch::I386, vendor: "pc".to_string(), os: OS::Windows, abi: ABI::MSVC },
    ]
}

pub fn parse_target_list(s: &str) -> Vec<TargetTriple> {
    s.split(',')
        .map(|t| TargetTriple::from_str(t.trim()))
        .collect()
}