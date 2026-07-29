use aly_rust::native::cocoa_backend::CocoaBackend;
use aly_rust::gui::{GuiBackend, register_backend};

fn main() {
    println!("Iniciando teste Cocoa GUI (macOS)... 🚀 Olá Mundo! ãõçé áéíóú");
    let mut backend = CocoaBackend;
    register_backend(Box::new(CocoaBackend));

    let win_id = backend.create_window("Cocoa Test 🚀 Olá", 600, 400);
    let lbl_id = backend.create_label("Texto inicial com acentos: áéíóú ãõç 🚀");
    let input_id = backend.create_input("Digite algo aqui...");
    let btn_id = backend.create_button("Clique em mim!");

    backend.insert_widget(&win_id, &lbl_id);
    backend.insert_widget(&win_id, &input_id);
    backend.insert_widget(&win_id, &btn_id);

    backend.on_event(&btn_id, "onClick", "on_button_click".to_owned());
    backend.on_event(&input_id, "onChange", "on_input_change".to_owned());

    println!("Botão pressionado / Input alterado configurados.");
    println!("Widgets criados. Executando loop da aplicação...");
    backend.run();
}
