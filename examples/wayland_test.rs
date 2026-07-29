// examples/wayland_test.rs

use Aly::gui::GuiBackend;
use Aly::native::wayland_backend::WaylandBackend;

fn main() {
    // Criar instância do backend Wayland
    let mut backend = WaylandBackend::new();

    // Criar uma janela usando o trait GuiBackend
    let window_id = backend.create_window("Test Window", 800, 600);
    
    println!("Window created with ID: {:?}", window_id);

    // Iniciar loop de eventos
    backend.run();
}