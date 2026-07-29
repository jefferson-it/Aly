#!/usr/bin/env rust

// src/system/notification.rs

// Linux (DBus)
#[cfg(target_os = "linux")]
mod linux_notification {
    use dbus::Message as DbusMessage;
    use dbus::Connection;
    
    pub struct Notification {
        dbus_connection: Connection,
        title: String,
        body: String,
    }
    
    impl Notification {
        pub fn new(title: &str, body: &str) -> Self {
            let connection = Connection::new_render_default().unwrap();
            Notification {
                dbus_connection: connection,
                title: title.to_string(),
                body: body.to_string(),
            }
        }
        
        pub fn show(&self) -> Result<(), String> {
            // Enviar notificao via DBus
            Ok(())
        }
    }
}


// Windows (Toast)
#[cfg(target_os = "windows")]
mod windows_notification {
    use windows_rs::wina::{ToastNotificationManager, ToastNotification};
    
    pub struct Notification {
        title: String,
        body: String,
    }
    
    impl Notification {
        pub fn new(title: &str, body: &str) -> Self {
            Notification {
                title: title.to_string(),
                body: body.to_string(),
            }
        }
        
        pub fn show(&self) -> Result<(), String> {
            // Criar e mostrar notificação Toast
            Ok(())
        }
    }
}


// macOS (UNUserNotificationCenter)
#[cfg(target_os = "macos")]
mod macos_notification {
    use objc::{id, selector, Class};
    
    pub struct Notification {
        title: String,
        body: String,
    }
    
    impl Notification {
        pub fn new(title: &str, body: &str) -> Self {
            Notification {
                title: title.to_string(),
                body: body.to_string(),
            }
        }
        
        pub fn show(&self) -> Result<(), String> {
            // Mostrar notificação via UNUserNotificationCenter
            Ok(())
        }
    }
}

// API pública
pub struct Notification {
    
}

impl Notification {
    pub fn new(title: &str, body: &str) -> Self {
        match std::env::consts::OS {
            "linux" => Notification::linux_notification::Notification::new(title, body),
            "windows" => Notification::windows_notification::Notification::new(title, body),
            "macos" => Notification::macos_notification::Notification::new(title, body),
            _ => panic!(\"Unsupported OS\"),
        }
    }
    
    pub fn show(&self) -> Result<(), String> {
        match self {
            Notification::linux_notification::Notification { .. } => self.linux_notification::Notification::show(),
            Notification::windows_notification::Notification { .. } => self.windows_notification::Notification::show(),
            Notification::macos_notification::Notification { .. } => self.macos_notification::Notification::show(),
        }
    }
}