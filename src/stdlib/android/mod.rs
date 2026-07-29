// Android stdlib module - provides Android APIs to Aly scripts

#[cfg(feature = "android")]
pub mod android {
    use crate::{
        aly::get_runtime,
        native::jni::{self, android::*},
        native::types::{ValueData, Validator},
        error::AlyError,
    };
    use std::sync::OnceLock;
    use std::collections::HashMap;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;
    use ::jni::objects::GlobalRef;
    use ::jni::objects::JValue;

    static INITIALIZED: OnceLock<bool> = OnceLock::new();
    static OBJECT_REGISTRY: Lazy<Mutex<HashMap<String, GlobalRef>>> = Lazy::new(|| Mutex::new(HashMap::new()));
    static mut NEXT_ID: usize = 1;

    fn register_object(obj: ::jni::objects::JObject) -> Option<String> {
        let env = jni::get_jvm()?.get_env().ok()?;
        let gref = env.new_global_ref(obj).ok()?;
        let id = unsafe {
            let current = NEXT_ID;
            NEXT_ID += 1;
            format!("android_obj_{}", current)
        };
        OBJECT_REGISTRY.lock().unwrap().insert(id.clone(), gref);
        Some(id)
    }

    fn get_object(id: &str) -> Option<::jni::objects::GlobalRef> {
        OBJECT_REGISTRY.lock().unwrap().get(id).cloned()
    }

    pub fn register_android_functions() {
        INITIALIZED.get_or_init(|| {
            let run = get_runtime();
            
            // Logging functions
            run.datas.push(crate::native::vars::Var::new(
                "logDebug".to_string(),
                log_debug as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "logInfo".to_string(),
                log_info as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "logWarn".to_string(),
                log_warn as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "logError".to_string(),
                log_error as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Toast
            run.datas.push(crate::native::vars::Var::new(
                "showToast".to_string(),
                show_toast as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // System info
            run.datas.push(crate::native::vars::Var::new(
                "getDeviceModel".to_string(),
                get_device_model as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getDeviceManufacturer".to_string(),
                get_device_manufacturer as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getAndroidVersion".to_string(),
                get_android_version as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Context functions
            run.datas.push(crate::native::vars::Var::new(
                "getPackageName".to_string(),
                get_package_name as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getFilesDir".to_string(),
                get_files_dir as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getCacheDir".to_string(),
                get_cache_dir as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Vibration
            run.datas.push(crate::native::vars::Var::new(
                "vibrate".to_string(),
                vibrate as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Screen metrics
            run.datas.push(crate::native::vars::Var::new(
                "getScreenWidth".to_string(),
                get_screen_width as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getScreenHeight".to_string(),
                get_screen_height as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getScreenDensity".to_string(),
                get_screen_density as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Intents
            run.datas.push(crate::native::vars::Var::new(
                "startActivity".to_string(),
                start_activity as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "sendSms".to_string(),
                send_sms as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // SharedPreferences
            run.datas.push(crate::native::vars::Var::new(
                "savePref".to_string(),
                save_pref as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "getPref".to_string(),
                get_pref as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Widgets
            run.datas.push(crate::native::vars::Var::new(
                "createView".to_string(),
                create_view as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "createButton".to_string(),
                create_button as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "createTextView".to_string(),
                create_text_view as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "createLinearLayout".to_string(),
                create_linear_layout as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "addView".to_string(),
                add_view as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Bluetooth
            run.datas.push(crate::native::vars::Var::new(
                "getBluetoothStatus".to_string(),
                get_bluetooth_status as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "enableBluetooth".to_string(),
                enable_bluetooth as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "disableBluetooth".to_string(),
                disable_bluetooth as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // NFC
            run.datas.push(crate::native::vars::Var::new(
                "getNfcStatus".to_string(),
                get_nfc_status as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "enableNfcDispatch".to_string(),
                enable_nfc_dispatch as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // GPS
            run.datas.push(crate::native::vars::Var::new(
                "getLastKnownLocation".to_string(),
                get_last_known_location as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "requestLocationUpdates".to_string(),
                request_location_updates as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Camera
            run.datas.push(crate::native::vars::Var::new(
                "getCameraIds".to_string(),
                get_camera_ids as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "openCameraDevice".to_string(),
                open_camera_device as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Sensors
            run.datas.push(crate::native::vars::Var::new(
                "getSensorList".to_string(),
                get_sensor_list as fn(String) -> Box<dyn Validator>,
                false,
            ));
            run.datas.push(crate::native::vars::Var::new(
                "registerSensorListener".to_string(),
                register_sensor_listener as fn(String) -> Box<dyn Validator>,
                false,
            ));

            // Cross-platform
            run.datas.push(crate::native::vars::Var::new(
                "getPlatformOS".to_string(),
                get_platform_os as fn(String) -> Box<dyn Validator>,
                false,
            ));

            true
        });
    }

    // Implementation functions that call JNI
    fn log_debug(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let tag = crate::native::std::arg(&args, 0);
        let msg = crate::native::std::arg(&args, 1);
        let _ = jni::android::log_debug(&tag, &msg);
        Box::new(ValueData::Option(None))
    }

    fn log_info(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let tag = crate::native::std::arg(&args, 0);
        let msg = crate::native::std::arg(&args, 1);
        let _ = jni::android::log_info(&tag, &msg);
        Box::new(ValueData::Option(None))
    }

    fn log_warn(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let tag = crate::native::std::arg(&args, 0);
        let msg = crate::native::std::arg(&args, 1);
        let _ = jni::android::log_warn(&tag, &msg);
        Box::new(ValueData::Option(None))
    }

    fn log_error(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let tag = crate::native::std::arg(&args, 0);
        let msg = crate::native::std::arg(&args, 1);
        let _ = jni::android::log_error(&tag, &msg);
        Box::new(ValueData::Option(None))
    }

    fn show_toast(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 3);
        let _context = crate::native::std::arg(&args, 0);
        let message = crate::native::std::arg(&args, 1);
        let duration = crate::native::std::arg(&args, 2);
        let dur = if duration == "long" { 1 } else { 0 };
        let _ = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let _ = jni::android::show_toast(&context, &message, dur);
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    fn get_device_model(_args: String) -> Box<dyn Validator> {
        let model = jni::android::get_device_model().unwrap_or_default();
        Box::new(ValueData::String(model))
    }

    fn get_device_manufacturer(_args: String) -> Box<dyn Validator> {
        let manufacturer = jni::android::get_device_manufacturer().unwrap_or_default();
        Box::new(ValueData::String(manufacturer))
    }

    fn get_android_version(_args: String) -> Box<dyn Validator> {
        let version = jni::android::get_version_sdk_int().unwrap_or(0).to_string();
        Box::new(ValueData::String(version))
    }

    fn get_package_name(_args: String) -> Box<dyn Validator> {
        let name = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            jni::android::get_package_name(&context)
        }).unwrap_or_default();
        Box::new(ValueData::String(name))
    }

    fn get_files_dir(_args: String) -> Box<dyn Validator> {
        let path = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let file = jni::android::get_files_dir(&context)?;
            let path_obj = env.call_method(&file, "getAbsolutePath", "()Ljava/lang/String;", &[]).ok()?.l().ok()?;
            let path_jstr: ::jni::objects::JString = path_obj.into();
            let s = env.get_string(&path_jstr).ok()?;
            Some(String::from(s))
        }).unwrap_or_default();
        Box::new(ValueData::String(path))
    }

    fn get_cache_dir(_args: String) -> Box<dyn Validator> {
        let path = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let file = jni::android::get_cache_dir(&context)?;
            let path_obj = env.call_method(&file, "getAbsolutePath", "()Ljava/lang/String;", &[]).ok()?.l().ok()?;
            let path_jstr: ::jni::objects::JString = path_obj.into();
            let s = env.get_string(&path_jstr).ok()?;
            Some(String::from(s))
        }).unwrap_or_default();
        Box::new(ValueData::String(path))
    }

    fn vibrate(milliseconds: String) -> Box<dyn Validator> {
        let ms: i64 = milliseconds.parse().unwrap_or(100);
        let _ = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let vibrator = jni::android::get_system_service(&context, "vibrator")?;
            jni::android::vibrate(&vibrator, ms);
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    fn get_screen_width(_args: String) -> Box<dyn Validator> {
        let width = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            jni::android::get_screen_width(&context)
        }).unwrap_or(0);
        Box::new(ValueData::Int(width))
    }

    fn get_screen_height(_args: String) -> Box<dyn Validator> {
        let height = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            jni::android::get_screen_height(&context)
        }).unwrap_or(0);
        Box::new(ValueData::Int(height))
    }

    fn get_screen_density(_args: String) -> Box<dyn Validator> {
        let density = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            jni::android::get_density(&context)
        }).unwrap_or(1.0);
        Box::new(ValueData::Float(density))
    }

    fn start_activity(class_name: String) -> Box<dyn Validator> {
        let _ = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let intent = jni::android::create_intent_with_class(&context, &class_name)?;
            jni::android::start_activity(&context, &intent);
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    fn send_sms(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let number = crate::native::std::arg(&args, 0);
        let text = crate::native::std::arg(&args, 1);
        let _ = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            jni::android::send_sms(&context, &number, &text);
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    fn save_pref(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let key = crate::native::std::arg(&args, 0);
        let value = crate::native::std::arg(&args, 1);
        let _ = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let prefs = jni::android::get_shared_preferences(&context, "AlyPrefs", 0)?;
            let editor = env.call_method(&prefs, "edit", "()Landroid/content/SharedPreferences$Editor;", &[]).ok()?.l().ok()?;
            let key_str = env.new_string(key).ok()?;
            let val_str = env.new_string(value).ok()?;
            let editor_with_str = env.call_method(&editor, "putString", "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/SharedPreferences$Editor;", &[(&key_str).into(), (&val_str).into()]).ok()?.l().ok()?;
            let _ = env.call_method(&editor_with_str, "apply", "()V", &[]).ok()?;
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    fn get_pref(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let key = crate::native::std::arg(&args, 0);
        let default_value = crate::native::std::arg(&args, 1);
        let value = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let prefs = jni::android::get_shared_preferences(&context, "AlyPrefs", 0)?;
            let key_str = env.new_string(key).ok()?;
            let def_str = env.new_string(default_value.clone()).ok()?;
            let val_obj = env.call_method(&prefs, "getString", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", &[(&key_str).into(), (&def_str).into()]).ok()?.l().ok()?;
            let val_jstr: ::jni::objects::JString = val_obj.into();
            let s = env.get_string(&val_jstr).ok()?;
            Some(String::from(s))
        }).unwrap_or(default_value);
        Box::new(ValueData::String(value))
    }

    // Widgets
    fn create_view(_args: String) -> Box<dyn Validator> {
        let id = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let view = jni::new_object(jni::android::VIEW_CLASS, "(Landroid/content/Context;)V", &[(&context).into()])?;
            register_object(view)
        });
        Box::new(ValueData::String(id.unwrap_or_default()))
    }

    fn create_button(_args: String) -> Box<dyn Validator> {
        let id = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let btn = jni::new_object(jni::android::BUTTON_CLASS, "(Landroid/content/Context;)V", &[(&context).into()])?;
            register_object(btn)
        });
        Box::new(ValueData::String(id.unwrap_or_default()))
    }

    fn create_text_view(_args: String) -> Box<dyn Validator> {
        let id = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let tv = jni::new_object(jni::android::TEXT_VIEW_CLASS, "(Landroid/content/Context;)V", &[(&context).into()])?;
            register_object(tv)
        });
        Box::new(ValueData::String(id.unwrap_or_default()))
    }

    fn create_linear_layout(_args: String) -> Box<dyn Validator> {
        let id = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let layout = jni::new_object(jni::android::LINEAR_LAYOUT_CLASS, "(Landroid/content/Context;)V", &[(&context).into()])?;
            register_object(layout)
        });
        Box::new(ValueData::String(id.unwrap_or_default()))
    }

    fn add_view(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 2);
        let layout_id = crate::native::std::arg(&args, 0);
        let view_id = crate::native::std::arg(&args, 1);
        
        let _ = jni::with_env_opt(|env| {
            let layout_ref = get_object(&layout_id)?;
            let view_ref = get_object(&view_id)?;
            let _ = env.call_method(layout_ref.as_obj(), "addView", "(Landroid/view/View;)V", &[view_ref.as_obj().into()]);
            Some(())
        });
        Box::new(ValueData::Option(None))
    }

    // Bluetooth
    fn get_bluetooth_status(_args: String) -> Box<dyn Validator> {
        let status = jni::with_env_opt(|_env| {
            let adapter = jni::android::get_bluetooth_adapter()?;
            let enabled = jni::android::is_bluetooth_enabled(&adapter).unwrap_or(false);
            Some(if enabled { "enabled" } else { "disabled" })
        }).unwrap_or("not_supported");
        Box::new(ValueData::String(status.to_string()))
    }

    fn enable_bluetooth(_args: String) -> Box<dyn Validator> {
        let success = jni::with_env_opt(|_env| {
            let adapter = jni::android::get_bluetooth_adapter()?;
            jni::android::enable_bluetooth(&adapter)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    fn disable_bluetooth(_args: String) -> Box<dyn Validator> {
        let success = jni::with_env_opt(|_env| {
            let adapter = jni::android::get_bluetooth_adapter()?;
            jni::android::disable_bluetooth(&adapter)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    // NFC
    fn get_nfc_status(_args: String) -> Box<dyn Validator> {
        let status = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let adapter = jni::android::get_nfc_adapter(&context)?;
            let enabled = jni::android::is_nfc_enabled(&adapter).unwrap_or(false);
            Some(if enabled { "enabled" } else { "disabled" })
        }).unwrap_or("not_supported");
        Box::new(ValueData::String(status.to_string()))
    }

    fn enable_nfc_dispatch(_args: String) -> Box<dyn Validator> {
        let success = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let adapter = jni::android::get_nfc_adapter(&context)?;
            let activity = &context; 
            let filters = ::jni::objects::JObject::null();
            let tech_lists = ::jni::objects::JObject::null();
            jni::android::enable_nfc_foreground_dispatch(&adapter, activity, &filters, &tech_lists)?;
            Some(true)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    // GPS
    fn get_last_known_location(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 1);
        let provider = crate::native::std::arg(&args, 0);
        let loc_str = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_location_manager(&context)?;
            let location = jni::android::get_last_known_location(&manager, &provider)?;
            let lat = env.call_method(&location, "getLatitude", "()D", &[]).ok()?.d().ok()?;
            let lon = env.call_method(&location, "getLongitude", "()D", &[]).ok()?.d().ok()?;
            Some(format!("{},{}", lat, lon))
        }).unwrap_or_else(|| "unknown".to_string());
        Box::new(ValueData::String(loc_str))
    }

    fn request_location_updates(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 1);
        let provider = crate::native::std::arg(&args, 0);
        let success = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_location_manager(&context)?;
            let listener = ::jni::objects::JObject::null();
            jni::android::request_location_updates(&manager, &provider, 1000, 1.0, &listener)?;
            Some(true)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    // Camera
    fn get_camera_ids(_args: String) -> Box<dyn Validator> {
        let ids_str = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_camera_manager(&context)?;
            let list = jni::android::get_camera_id_list(&manager)?;
            let array: &::jni::objects::JObjectArray = unsafe { std::mem::transmute(&list) };
            let len = env.get_array_length(array).unwrap_or(0);
            let mut ids = Vec::new();
            for i in 0..len {
                let obj = env.get_object_array_element(array, i).ok()?;
                let s_jstr = ::jni::objects::JString::from(obj);
                let s = env.get_string(&s_jstr).ok()?;
                ids.push(String::from(s.to_str().unwrap_or("")));
            }
            Some(ids.join(","))
        }).unwrap_or_default();
        Box::new(ValueData::String(ids_str))
    }

    fn open_camera_device(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 1);
        let camera_id = crate::native::std::arg(&args, 0);
        let success = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_camera_manager(&context)?;
            let callback = ::jni::objects::JObject::null();
            let handler = ::jni::objects::JObject::null();
            jni::android::open_camera(&manager, &camera_id, &callback, &handler)?;
            Some(true)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    // Sensors
    fn get_sensor_list(_args: String) -> Box<dyn Validator> {
        let sensors_str = jni::with_env_opt(|env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_sensor_manager(&context)?;
            let list = jni::android::get_sensor_list(&manager, -1)?;
            let len = env.call_method(&list, "size", "()I", &[]).ok()?.i().ok()?;
            let mut names = Vec::new();
            for i in 0..len {
                let sensor = env.call_method(&list, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)]).ok()?.l().ok()?;
                let name_obj = env.call_method(&sensor, "getName", "()Ljava/lang/String;", &[]).ok()?.l().ok()?;
                let name_obj = env.call_method(&sensor, "getName", "()Ljava/lang/String;", &[]).ok()?.l().ok()?;
                let name_jstr = ::jni::objects::JString::from(name_obj);
                let s = env.get_string(&name_jstr).ok()?;
                names.push(String::from(s.to_str().unwrap_or("")));
            }
            Some(names.join(","))
        }).unwrap_or_default();
        Box::new(ValueData::String(sensors_str))
    }

    fn register_sensor_listener(args_str: String) -> Box<dyn Validator> {
        let args = crate::native::std::split_args(&args_str, 1);
        let sensor_type_str = crate::native::std::arg(&args, 0);
        let sensor_type: i32 = sensor_type_str.parse().unwrap_or(1);
        let success = jni::with_env_opt(|_env| {
            let context = jni::android::get_context()?;
            let manager = jni::android::get_sensor_manager(&context)?;
            let sensor = jni::android::get_default_sensor(&manager, sensor_type)?;
            let listener = ::jni::objects::JObject::null();
            let success = jni::android::register_listener(&manager, &listener, &sensor, 3).unwrap_or(false);
            Some(success)
        }).unwrap_or(false);
        Box::new(ValueData::Bool(success))
    }

    // Cross-platform support
    fn get_platform_os(_args: String) -> Box<dyn Validator> {
        Box::new(ValueData::String("android".to_string()))
    }
}

#[cfg(not(feature = "android"))]
pub fn init_android_stdlib() {
    // No-op when android feature is not enabled
}

#[cfg(feature = "android")]
pub use android::register_android_functions as init_android_stdlib;
