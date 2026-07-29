//! Win32 Backend Test Example
//! 
//! This example demonstrates the Win32/Win64 native backend for the GUI module.
//! It creates a window with a button, label, and input field.
//!
//! Run with: cargo run --example win32_test --features "gui windows"

#[cfg(target_os = "windows")]
fn main() {
    use aly::gui::{with_backend, register_backend};
    use aly::native::win32_backend::Win32Backend;
    
    println!("Starting Win32 GUI test...");
    
    // Register the Win32 backend
    register_backend(Box::new(Win32Backend));
    
    with_backend(|backend| {
        // Create main window
        let window_id = backend.create_window("Aly Win32 Test", 600, 400);
        println!("Created window: {}", window_id);
        
        // Create a label
        let label_id = backend.create_label("Enter text below:");
        println!("Created label: {}", label_id);
        
        // Create an input field
        let input_id = backend.create_input("Type something...");
        println!("Created input: {}", input_id);
        
        // Create a button
        let button_id = backend.create_button("Click Me!");
        println!("Created button: {}", button_id);
        
        // Set up event handlers
        backend.on_event(&button_id, "onClick", "on_button_click".to_string());
        backend.on_event(&input_id, "onChange", "on_input_change".to_string());
        backend.on_event(&window_id, "onClose", "on_window_close".to_string());
        
        // Insert widgets into window
        backend.insert_widget(&window_id, &label_id);
        backend.insert_widget(&window_id, &input_id);
        backend.insert_widget(&window_id, &button_id);
        
        // Run the event loop
        backend.run();
    });
    
    println!("GUI test completed.");
}

// Dummy main for non-Windows targets to allow compilation
#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This example only works on Windows. Please run on Windows target.");
    println!("Use: cargo run --example win32_test --features \"gui windows\" --target x86_64-pc-windows-msvc");
}