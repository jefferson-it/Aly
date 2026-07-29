// iOS Native Module
// Provides Cocoa/Swift integration for iOS applications

#[cfg(target_os = "ios")]
pub mod hardware;

#[cfg(target_os = "ios")]
pub mod ios_backend;