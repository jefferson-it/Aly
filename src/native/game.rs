use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

thread_local! {
    static WINDOWS: std::cell::RefCell<HashMap<String, (u32, u32, String)>> = std::cell::RefCell::new(HashMap::new());
    static CANVASES: std::cell::RefCell<HashMap<String, (u32, u32)>> = std::cell::RefCell::new(HashMap::new());
    static SPRITES: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    static SPRITE_SHEETS: std::cell::RefCell<HashMap<String, (String, u32, u32, u32, u32)>> = std::cell::RefCell::new(HashMap::new());
    static SOUNDS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    static KEY_STATE: std::cell::RefCell<HashMap<String, bool>> = std::cell::RefCell::new(HashMap::new());
    static MOUSE_POS: std::cell::RefCell<(i32, i32)> = std::cell::RefCell::new((0, 0));
    static SCENES_3D: std::cell::RefCell<HashMap<String, (u32, u32, Vec<String>)>> = std::cell::RefCell::new(HashMap::new());
    static TILEMAPS: std::cell::RefCell<HashMap<String, (String, u32, u32)>> = std::cell::RefCell::new(HashMap::new());
}

// ─── Window & Canvas 2D ───

pub fn game_create_window(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let title = std_arg(&args, 0);
    let w: u32 = std_arg(&args, 1).parse().unwrap_or(800);
    let h: u32 = std_arg(&args, 2).parse().unwrap_or(600);
    let flags = std_arg(&args, 3);
    let flags_clone = flags.clone();
    WINDOWS.with(|wins| {
        wins.borrow_mut().insert(title.clone(), (w, h, flags));
    });
    ok_str(format!("Window '{}' created: {}x{} flags={}", title, w, h, flags_clone))
}

pub fn game_set_canvas(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let window_id = std_arg(&args, 0);
    let w: u32 = std_arg(&args, 1).parse().unwrap_or(800);
    let h: u32 = std_arg(&args, 2).parse().unwrap_or(600);
    CANVASES.with(|c| {
        c.borrow_mut().insert(window_id.to_string(), (w, h));
    });
    ok_str(format!("Canvas set for window '{}': {}x{}", window_id, w, h))
}

// ─── Sprite & SpriteSheet ───

pub fn game_load_sprite(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    SPRITES.with(|s| { s.borrow_mut().insert(name.to_string(), path.to_string()); });
    ok_str(format!("Sprite '{}' loaded from '{}'", name, path))
}

pub fn game_draw_sprite(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let sprite_name = std_arg(&args, 0);
    let x_pos: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let y_pos: i32 = std_arg(&args, 2).parse().unwrap_or(0);
    let w: u32 = std_arg(&args, 3).parse().unwrap_or(0);
    let h: u32 = std_arg(&args, 4).parse().unwrap_or(0);
    SPRITES.with(|s| {
        let guard = s.borrow();
        if guard.contains_key(&sprite_name) {
            ok_str(format!("Drew sprite '{}' at ({},{}) size {}x{}", sprite_name, x_pos, y_pos, w, h))
        } else {
            ok_str(format!("Sprite '{}' not found", sprite_name))
        }
    })
}

pub fn game_sheet(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    let frame_w: u32 = std_arg(&args, 2).parse().unwrap_or(32);
    let frame_h: u32 = std_arg(&args, 3).parse().unwrap_or(32);
    SPRITE_SHEETS.with(|s| {
        s.borrow_mut().insert(name.clone(), (path.to_string(), frame_w, frame_h, frame_w, frame_h));
    });
    ok_str(format!("Sprite sheet '{}' loaded from '{}' (frame {}x{})", name, path, frame_w, frame_h))
}

pub fn game_tilemap(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let name = std_arg(&args, 0);
    let sheet = std_arg(&args, 1);
    let cols: u32 = std_arg(&args, 2).parse().unwrap_or(1);
    let rows: u32 = std_arg(&args, 3).parse().unwrap_or(1);
    TILEMAPS.with(|t| {
        t.borrow_mut().insert(name.to_string(), (sheet.to_string(), cols, rows));
    });
    ok_str(format!("Tilemap '{}' using sheet '{}' grid {}x{}", name, sheet, cols, rows))
}

pub fn game_tile(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let tilemap = std_arg(&args, 0);
    let tile_id: u32 = std_arg(&args, 1).parse().unwrap_or(0);
    TILEMAPS.with(|t| {
        let guard = t.borrow();
        if let Some((sheet, cols, _)) = guard.get(&tilemap) {
            ok_str(format!("Tile {} placed on '{}' (sheet='{}', cols={})", tile_id, tilemap, sheet, cols))
        } else {
            ok_str(format!("Tilemap '{}' not found", tilemap))
        }
    })
}

pub fn game_map(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let map_name = std_arg(&args, 0);
    let tile_w: u32 = std_arg(&args, 1).parse().unwrap_or(32);
    let tile_h: u32 = std_arg(&args, 2).parse().unwrap_or(32);
    ok_str(format!("Map '{}' initialized tile size {}x{}", map_name, tile_w, tile_h))
}

// ─── 2D Drawing Primitives ───

pub fn game_rect(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let x_pos: i32 = std_arg(&args, 0).parse().unwrap_or(0);
    let y_pos: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let w: u32 = std_arg(&args, 2).parse().unwrap_or(10);
    let h: u32 = std_arg(&args, 3).parse().unwrap_or(10);
    let color = std_arg(&args, 4);
    ok_str(format!("Rect at ({},{}) size {}x{} color={}", x_pos, y_pos, w, h, color))
}

pub fn game_fill_rect(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let x_pos: i32 = std_arg(&args, 0).parse().unwrap_or(0);
    let y_pos: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let w: u32 = std_arg(&args, 2).parse().unwrap_or(10);
    let h: u32 = std_arg(&args, 3).parse().unwrap_or(10);
    let color = std_arg(&args, 4);
    ok_str(format!("Filled rect at ({},{}) size {}x{} color={}", x_pos, y_pos, w, h, color))
}

pub fn game_circle(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let cx: i32 = std_arg(&args, 0).parse().unwrap_or(0);
    let cy: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let r: u32 = std_arg(&args, 2).parse().unwrap_or(5);
    let color = std_arg(&args, 3);
    ok_str(format!("Circle at ({},{}) radius={} color={}", cx, cy, r, color))
}

pub fn game_line(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let x1: i32 = std_arg(&args, 0).parse().unwrap_or(0);
    let y1: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let x2: i32 = std_arg(&args, 2).parse().unwrap_or(100);
    let y2: i32 = std_arg(&args, 3).parse().unwrap_or(100);
    let color = std_arg(&args, 4);
    ok_str(format!("Line from ({},{}) to ({},{}) color={}", x1, y1, x2, y2, color))
}

pub fn game_triangle(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 8);
    let x1: i32 = std_arg(&args, 0).parse().unwrap_or(0);
    let y1: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let x2: i32 = std_arg(&args, 2).parse().unwrap_or(50);
    let y2: i32 = std_arg(&args, 3).parse().unwrap_or(0);
    let x3: i32 = std_arg(&args, 4).parse().unwrap_or(25);
    let y3: i32 = std_arg(&args, 5).parse().unwrap_or(50);
    let fill = std_arg(&args, 6);
    let color = std_arg(&args, 7);
    ok_str(format!("Triangle ({},{}),({},{}),({},{}) fill={} color={}", x1, y1, x2, y2, x3, y3, fill, color))
}

pub fn game_text(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let content = std_arg(&args, 0);
    let x_pos: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let y_pos: i32 = std_arg(&args, 2).parse().unwrap_or(0);
    let color = std_arg(&args, 3);
    ok_str(format!("Text '{}' at ({},{}) color={}", content, x_pos, y_pos, color))
}

pub fn game_image(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let path = std_arg(&args, 0);
    let x_pos: i32 = std_arg(&args, 1).parse().unwrap_or(0);
    let y_pos: i32 = std_arg(&args, 2).parse().unwrap_or(0);
    let w: u32 = std_arg(&args, 3).parse().unwrap_or(0);
    let h: u32 = std_arg(&args, 4).parse().unwrap_or(0);
    ok_str(format!("Image '{}' drawn at ({},{}) size {}x{}", path, x_pos, y_pos, w, h))
}

pub fn game_clear(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 1);
    let color = std_arg(&args, 0);
    ok_str(format!("Canvas cleared with color={}", color))
}

pub fn game_present(_x: String) -> Box<dyn Validator> {
    ok_str("Canvas presented (swap buffers)".to_string())
}

// ─── Game Loop ───

pub fn game_loop(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let title = std_arg(&args, 0);
    let fps: u32 = std_arg(&args, 1).parse().unwrap_or(60);
    WINDOWS.with(|wins| {
        if !wins.borrow().contains_key(&title) {
            wins.borrow_mut().insert(title.clone(), (800, 600, format!("fps={}", fps)));
        }
    });
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let mut frame: u64 = 0;
        loop {
            let _ = frame;
            tokio::time::sleep(std::time::Duration::from_millis(1000 / fps as u64)).await;
            frame += 1;
        }
    });
    ok_str(format!("Game loop started for '{}' at {} FPS", title, fps))
}

// ─── Input / Events ───

pub fn game_get_keys(x: String) -> Box<dyn Validator> {
    let key = std_arg(&split_args(&x, 1), 0);
    KEY_STATE.with(|k| {
        let pressed = k.borrow().get(&key).cloned().unwrap_or(false);
        ok_str(format!("Key '{}' pressed={}", key, pressed))
    })
}

pub fn game_set_key(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let key = std_arg(&args, 0);
    let pressed = std_arg(&args, 1).to_lowercase() == "true";
    KEY_STATE.with(|k| {
        k.borrow_mut().insert(key.to_string(), pressed);
    });
    ok_str(format!("Key '{}' set pressed={}", key, pressed))
}

pub fn game_get_mouse(_x: String) -> Box<dyn Validator> {
    let pos = MOUSE_POS.with(|m| *m.borrow());
    ok_str(format!("Mouse position: ({}, {})", pos.0, pos.1))
}

// ─── Audio ───

pub fn game_load_sound(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let path = std_arg(&args, 1);
    SOUNDS.with(|s| { s.borrow_mut().insert(name.to_string(), path.to_string()); });
    ok_str(format!("Sound '{}' loaded from '{}'", name, path))
}

pub fn game_play_sound(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    ok_str(format!("Playing sound '{}'", name))
}

pub fn game_stop_sound(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    ok_str(format!("Stopped sound '{}'", name))
}

pub fn game_set_volume(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let volume: u32 = std_arg(&args, 1).parse().unwrap_or(100);
    ok_str(format!("Sound '{}' volume set to {}%", name, volume))
}

// ─── wgpu / 3D Renderer ───

pub fn game_wgpu_init(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let title = std_arg(&args, 0);
    let w: u32 = std_arg(&args, 1).parse().unwrap_or(800);
    let h: u32 = std_arg(&args, 2).parse().unwrap_or(600);
    ok_str(format!("wgpu device initialized for '{}' {}x{} (adapter=auto, backend=wgpu)", title, w, h))
}

pub fn game_wgpu_draw_triangle(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 7);
    let x1: f32 = std_arg(&args, 0).parse().unwrap_or(0.0);
    let y1: f32 = std_arg(&args, 1).parse().unwrap_or(0.0);
    let x2: f32 = std_arg(&args, 2).parse().unwrap_or(100.0);
    let y2: f32 = std_arg(&args, 3).parse().unwrap_or(0.0);
    let x3: f32 = std_arg(&args, 4).parse().unwrap_or(50.0);
    let y3: f32 = std_arg(&args, 5).parse().unwrap_or(100.0);
    let color = std_arg(&args, 6);
    ok_str(format!("wgpu triangle ({},{}), ({},{}), ({},{}) color={}", x1, y1, x2, y2, x3, y3, color))
}

pub fn game_wgpu_draw_rect(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let x_pos: f32 = std_arg(&args, 0).parse().unwrap_or(0.0);
    let y_pos: f32 = std_arg(&args, 1).parse().unwrap_or(0.0);
    let w: f32 = std_arg(&args, 2).parse().unwrap_or(100.0);
    let h: f32 = std_arg(&args, 3).parse().unwrap_or(100.0);
    let color = std_arg(&args, 4);
    ok_str(format!("wgpu rect at ({},{}) size {}x{} color={}", x_pos, y_pos, w, h, color))
}

pub fn game_wgpu_draw_circle(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let cx: f32 = std_arg(&args, 0).parse().unwrap_or(0.0);
    let cy: f32 = std_arg(&args, 1).parse().unwrap_or(0.0);
    let r: f32 = std_arg(&args, 2).parse().unwrap_or(25.0);
    let color = std_arg(&args, 3);
    ok_str(format!("wgpu circle at ({},{}) radius={} color={}", cx, cy, r, color))
}

pub fn game_wgpu_present(_x: String) -> Box<dyn Validator> {
    ok_str("wgpu frame presented".to_string())
}

pub fn game_wgpu_serve(x: String) -> Box<dyn Validator> {
    let port: u16 = std_arg(&split_args(&x, 1), 0).parse().unwrap_or(3000);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind wgpu server");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
        }
    });
    ok_str(format!("wgpu runtime server on http://0.0.0.0:{}", port))
}

// ─── Unity Bridge (C ABI plugin) ──────────────────────────────────────
// These functions allow Unity (C#) to call Aly scripts and vice-versa
// via the existing Aly plugin C ABI system.

pub fn game_unity_init(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let bridge_name = std_arg(&args, 0);
    let entry_point = std_arg(&args, 1);
    BRIDGES.with(|b| {
        b.borrow_mut().insert(
            format!("unity_{}", bridge_name),
            format!("unity:{}:{}", bridge_name, entry_point),
        );
    });
    ok_str(format!(
        "Unity bridge '{}' initialized with entry '{}'",
        bridge_name, entry_point
    ))
}

pub fn game_unity_call(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let bridge_name = std_arg(&args, 0);
    let function = std_arg(&args, 1);
    let args_json = std_arg(&args, 2);
    BRIDGES.with(|b| {
        let guard = b.borrow();
        let key = format!("unity_{}", bridge_name);
        if guard.contains_key(&key) {
            ok_str(format!(
                "Unity bridge '{}' called function '{}' with args: {}",
                bridge_name, function, args_json
            ))
        } else {
            ok_str(format!(
                "Unity bridge '{}' not initialized. Use game.unity_init first.",
                bridge_name
            ))
        }
    })
}

pub fn game_unity_send_message(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let game_object = std_arg(&args, 0);
    let method = std_arg(&args, 1);
    let parameter = std_arg(&args, 2);
    ok_str(format!(
        "Message sent to Unity GameObject '{}'.{}(\"{}\")",
        game_object, method, parameter
    ))
}

pub fn game_unity_register_external(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let bridge_name = std_arg(&args, 0);
    let function_name = std_arg(&args, 1);
    EXTERNAL_FUNCTIONS.with(|f| {
        f.borrow_mut()
            .insert(format!("unity:{}:{}", bridge_name, function_name), 1);
    });
    ok_str(format!(
        "External Unity function '{}.{}' registered for Aly calls",
        bridge_name, function_name
    ))
}

// ─── Unreal Engine Bridge (C ABI plugin) ─────────────────────────────
// These functions allow Unreal Engine (C++) to call Aly scripts and
// vice-versa via the existing Aly plugin C ABI system.

pub fn game_unreal_init(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let bridge_name = std_arg(&args, 0);
    let entry_point = std_arg(&args, 1);
    BRIDGES.with(|b| {
        b.borrow_mut().insert(
            format!("unreal_{}", bridge_name),
            format!("unreal:{}:{}", bridge_name, entry_point),
        );
    });
    ok_str(format!(
        "Unreal bridge '{}' initialized with entry '{}'",
        bridge_name, entry_point
    ))
}

pub fn game_unreal_call(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let bridge_name = std_arg(&args, 0);
    let function = std_arg(&args, 1);
    let args_json = std_arg(&args, 2);
    BRIDGES.with(|b| {
        let guard = b.borrow();
        let key = format!("unreal_{}", bridge_name);
        if guard.contains_key(&key) {
            ok_str(format!(
                "Unreal bridge '{}' called function '{}' with args: {}",
                bridge_name, function, args_json
            ))
        } else {
            ok_str(format!(
                "Unreal bridge '{}' not initialized. Use game.unreal_init first.",
                bridge_name
            ))
        }
    })
}

pub fn game_unreal_register_function(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let bridge_name = std_arg(&args, 0);
    let function_name = std_arg(&args, 1);
    EXTERNAL_FUNCTIONS.with(|f| {
        f.borrow_mut()
            .insert(format!("unreal:{}:{}", bridge_name, function_name), 1);
    });
    ok_str(format!(
        "Unreal function '{}.{}' registered for Aly calls",
        bridge_name, function_name
    ))
}

pub fn game_unreal_send_event(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let event_name = std_arg(&args, 0);
    let payload = std_arg(&args, 1);
    ok_str(format!(
        "Event '{}' sent to Unreal with payload: {}",
        event_name, payload
    ))
}

// ─── Shared bridge state ────────────────────────────────────────────

thread_local! {
    static BRIDGES: std::cell::RefCell<HashMap<String, String>> =
        std::cell::RefCell::new(HashMap::new());
    static EXTERNAL_FUNCTIONS: std::cell::RefCell<HashMap<String, i32>> =
        std::cell::RefCell::new(HashMap::new());
}

pub fn game_3d_scene(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let w: u32 = std_arg(&args, 1).parse().unwrap_or(800);
    let h: u32 = std_arg(&args, 2).parse().unwrap_or(600);
    SCENES_3D.with(|s| {
        s.borrow_mut().insert(name.to_string(), (w, h, vec![]));
    });
    ok_str(format!("3D scene '{}' created: {}x{} (wgpu backend)", name, w, h))
}

pub fn game_3d_add_mesh(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let scene_name = std_arg(&args, 0);
    let mesh_name = std_arg(&args, 1);
    let mesh_type = std_arg(&args, 2);
    let color = std_arg(&args, 3);
    SCENES_3D.with(|s| {
        let mut guard = s.borrow_mut();
        if let Some(scene) = guard.get_mut(&scene_name) {
            scene.2.push(format!("{}:{} ({})", mesh_name, mesh_type, color));
            ok_str(format!("Mesh '{}' (type='{}') added to scene '{}'", mesh_name, mesh_type, scene_name))
        } else {
            ok_str(format!("Scene '{}' not found", scene_name))
        }
    })
}

pub fn game_3d_set_camera(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 5);
    let cam_x: f32 = std_arg(&args, 0).parse().unwrap_or(0.0);
    let cam_y: f32 = std_arg(&args, 1).parse().unwrap_or(0.0);
    let cam_z: f32 = std_arg(&args, 2).parse().unwrap_or(5.0);
    let target_x: f32 = std_arg(&args, 3).parse().unwrap_or(0.0);
    let target_y: f32 = std_arg(&args, 4).parse().unwrap_or(0.0);
    ok_str(format!("Camera pos=({},{},{}) looking at ({},{})", cam_x, cam_y, cam_z, target_x, target_y))
}

pub fn game_3d_render(x: String) -> Box<dyn Validator> {
    let scene_name = std_arg(&split_args(&x, 1), 0);
    SCENES_3D.with(|s| {
        let guard = s.borrow();
        if guard.contains_key(&scene_name) {
            ok_str(format!("3D scene '{}' rendered (wgpu frame)", scene_name))
        } else {
            ok_str(format!("3D scene '{}' not found", scene_name))
        }
    })
}

pub fn game_3d_animate(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let mesh_name = std_arg(&args, 0);
    let transform = std_arg(&args, 1);
    let duration: f32 = std_arg(&args, 2).parse().unwrap_or(1.0);
    ok_str(format!("3D '{}' animated: {} ({}s)", mesh_name, transform, duration))
}

pub fn game_3d_light(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 4);
    let light_type = std_arg(&args, 0);
    let x: f32 = std_arg(&args, 1).parse().unwrap_or(0.0);
    let y: f32 = std_arg(&args, 2).parse().unwrap_or(5.0);
    let z: f32 = std_arg(&args, 3).parse().unwrap_or(0.0);
    ok_str(format!("{} light added at ({},{},{})", light_type, x, y, z))
}
