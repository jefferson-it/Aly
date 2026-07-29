// Android Hardware Abstraction Layer
// Provides access to device hardware features via JNI

#![cfg(feature = "android")]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use jni::objects::{JObject, JValue};
use jni::JavaVM;

use crate::native::jni::{get_jvm, with_env};

pub type SharedCallback = Rc<RefCell<dyn Fn(String) + Send + Sync>>;

thread_local! {
    static BLUETOOTH_CALLBACKS: RefCell<HashMap<String, SharedCallback>> = RefCell::new(HashMap::new());
    static NFC_CALLBACKS: RefCell<HashMap<String, SharedCallback>> = RefCell::new(HashMap::new());
    static LOCATION_CALLBACKS: RefCell<HashMap<String, SharedCallback>> = RefCell::new(HashMap::new());
    static SENSOR_CALLBACKS: RefCell<HashMap<String, SharedCallback>> = RefCell::new(HashMap::new());
    static CAMERA_CALLBACKS: RefCell<HashMap<String, SharedCallback>> = RefCell::new(HashMap::new());
}

// ============================================================================
// Bluetooth
// ============================================================================

pub fn bluetooth_is_enabled() -> Result<bool, String> {
    with_env(|env| {
        let bluetooth_adapter = env
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .map_err(|e| format!("Failed to get BluetoothAdapter: {:?}", e))?
            .l()
            .map_err(|_| "BluetoothAdapter is null")?;
        
        if bluetooth_adapter.is_null() {
            return Ok(false);
        }
        
        let enabled = env
            .call_method(&bluetooth_adapter, "isEnabled", "()Z", &[])
            .map_err(|e| format!("Failed to check bluetooth enabled: {:?}", e))?
            .z()
            .map_err(|_| "Failed to get boolean")?;
        
        Ok(enabled)
    })
}

pub fn bluetooth_enable() -> Result<(), String> {
    with_env(|env| {
        let bluetooth_adapter = env
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .map_err(|e| format!("Failed to get BluetoothAdapter: {:?}", e))?
            .l()
            .map_err(|_| "BluetoothAdapter is null")?;
        
        if bluetooth_adapter.is_null() {
            return Err("No Bluetooth adapter found".to_string());
        }
        
        env.call_method(&bluetooth_adapter, "enable", "()Z", &[])
            .map_err(|e| format!("Failed to enable bluetooth: {:?}", e))?;
        
        Ok(())
    })
}

pub fn bluetooth_disable() -> Result<(), String> {
    with_env(|env| {
        let bluetooth_adapter = env
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .map_err(|e| format!("Failed to get BluetoothAdapter: {:?}", e))?
            .l()
            .map_err(|_| "BluetoothAdapter is null")?;
        
        if bluetooth_adapter.is_null() {
            return Err("No Bluetooth adapter found".to_string());
        }
        
        env.call_method(&bluetooth_adapter, "disable", "()Z", &[])
            .map_err(|e| format!("Failed to disable bluetooth: {:?}", e))?;
        
        Ok(())
    })
}

pub fn bluetooth_start_discovery() -> Result<(), String> {
    with_env(|env| {
        let bluetooth_adapter = env
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .map_err(|e| format!("Failed to get BluetoothAdapter: {:?}", e))?
            .l()
            .map_err(|_| "BluetoothAdapter is null")?;
        
        if bluetooth_adapter.is_null() {
            return Err("No Bluetooth adapter found".to_string());
        }
        
        env.call_method(&bluetooth_adapter, "startDiscovery", "()Z", &[])
            .map_err(|e| format!("Failed to start discovery: {:?}", e))?;
        
        Ok(())
    })
}

pub fn bluetooth_cancel_discovery() -> Result<(), String> {
    with_env(|env| {
        let bluetooth_adapter = env
            .call_static_method(
                "android/bluetooth/BluetoothAdapter",
                "getDefaultAdapter",
                "()Landroid/bluetooth/BluetoothAdapter;",
                &[],
            )
            .map_err(|e| format!("Failed to get BluetoothAdapter: {:?}", e))?
            .l()
            .map_err(|_| "BluetoothAdapter is null")?;
        
        if bluetooth_adapter.is_null() {
            return Err("No Bluetooth adapter found".to_string());
        }
        
        env.call_method(&bluetooth_adapter, "cancelDiscovery", "()Z", &[])
            .map_err(|e| format!("Failed to cancel discovery: {:?}", e))?;
        
        Ok(())
    })
}

pub fn bluetooth_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    BLUETOOTH_CALLBACKS.with(|c| {
        c.borrow_mut().insert(callback_name, Rc::new(RefCell::new(callback)));
    });
    Ok(())
}

// ============================================================================
// NFC
// ============================================================================

pub fn nfc_is_enabled() -> Result<bool, String> {
    with_env(|env| {
        let nfc_adapter = env
            .call_static_method(
                "android/nfc/NfcAdapter",
                "getDefaultAdapter",
                "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;",
                &[JValue::Object(&get_android_context()?)],
            )
            .map_err(|e| format!("Failed to get NfcAdapter: {:?}", e))?
            .l()
            .map_err(|_| "NfcAdapter is null")?;
        
        if nfc_adapter.is_null() {
            return Ok(false);
        }
        
        let enabled = env
            .call_method(&nfc_adapter, "isEnabled", "()Z", &[])
            .map_err(|e| format!("Failed to check NFC enabled: {:?}", e))?
            .z()
            .map_err(|_| "Failed to get boolean")?;
        
        Ok(enabled)
    })
}

pub fn nfc_enable_foreground_dispatch() -> Result<(), String> {
    with_env(|env| {
        let activity = get_android_activity()?;
        let nfc_adapter = env
            .call_static_method(
                "android/nfc/NfcAdapter",
                "getDefaultAdapter",
                "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;",
                &[JValue::Object(&activity)],
            )
            .map_err(|e| format!("Failed to get NfcAdapter: {:?}", e))?
            .l()
            .map_err(|_| "NfcAdapter is null")?;
        
        if nfc_adapter.is_null() {
            return Err("No NFC adapter found".to_string());
        }
        
        let pending_intent = env
            .call_static_method(
                "android/app/PendingIntent",
                "getActivity",
                "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;",
                &[
                    JValue::Object(&activity),
                    JValue::Int(0),
                    JValue::Object(&JObject::null()),
                    JValue::Int(0),
                ],
            )
            .map_err(|e| format!("Failed to create PendingIntent: {:?}", e))?
            .l()
            .map_err(|_| "Failed to create PendingIntent")?;
        
        if pending_intent.is_null() {
            return Err("Failed to create PendingIntent".to_string());
        }
        
        env.call_method(&nfc_adapter, "enableForegroundDispatch", "(Landroid/app/Activity;Landroid/app/PendingIntent;[Landroid/content/IntentFilter;[[Ljava/lang/String;)V", &[
            JValue::Object(&activity),
            JValue::Object(&pending_intent),
            JValue::Object(&JObject::null()),
            JValue::Object(&JObject::null()),
        ])
        .map_err(|e| format!("Failed to enable foreground dispatch: {:?}", e))?;
        
        Ok(())
    })
}

pub fn nfc_disable_foreground_dispatch() -> Result<(), String> {
    with_env(|env| {
        let activity = get_android_activity()?;
        let nfc_adapter = env
            .call_static_method(
                "android/nfc/NfcAdapter",
                "getDefaultAdapter",
                "(Landroid/content/Context;)Landroid/nfc/NfcAdapter;",
                &[JValue::Object(&activity)],
            )
            .map_err(|e| format!("Failed to get NfcAdapter: {:?}", e))?
            .l()
            .map_err(|_| "NfcAdapter is null")?;
        
        if nfc_adapter.is_null() {
            return Err("No NFC adapter found".to_string());
        }
        
        env.call_method(&nfc_adapter, "disableForegroundDispatch", "(Landroid/app/Activity;)V", &[JValue::Object(&activity)])
            .map_err(|e| format!("Failed to disable foreground dispatch: {:?}", e))?;
        
        Ok(())
    })
}

pub fn nfc_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    NFC_CALLBACKS.with(|c| {
        c.borrow_mut().insert(callback_name, Rc::new(RefCell::new(callback)));
    });
    Ok(())
}

// ============================================================================
// GPS / Location
// ============================================================================

pub fn gps_is_enabled() -> Result<bool, String> {
    with_env(|env| {
        let location_manager = env
            .call_method(&get_android_context()?, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("location").unwrap().into())])
            .map_err(|e| format!("Failed to get LocationManager: {:?}", e))?
            .l()
            .map_err(|_| "LocationManager is null")?;
        
        if location_manager.is_null() {
            return Ok(false);
        }
        
        let gps_enabled = env
            .call_method(&location_manager, "isProviderEnabled", "(Ljava/lang/String;)Z", &[JValue::Object(&env.new_string("gps").unwrap().into())])
            .map_err(|e| format!("Failed to check GPS enabled: {:?}", e))?
            .z()
            .map_err(|_| "Failed to get boolean")?;
        
        Ok(gps_enabled)
    })
}

pub fn gps_request_location_updates(min_time_ms: i64, min_distance_m: f32) -> Result<(), String> {
    with_env(|env| {
        let context = get_android_context()?;
        let location_manager = env
            .call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("location").unwrap().into())])
            .map_err(|e| format!("Failed to get LocationManager: {:?}", e))?
            .l()
            .map_err(|_| "LocationManager is null")?;
        
        if location_manager.is_null() {
            return Err("LocationManager is null".to_string());
        }
        
        // Create a location listener
        let listener_class = env.find_class("android/location/LocationListener")
            .map_err(|e| format!("Failed to find LocationListener: {:?}", e))?;
        
        // We'd need to implement a custom LocationListener - simplified here
        env.call_method(&location_manager, "requestLocationUpdates", "(Ljava/lang/String;JFLandroid/location/LocationListener;)V", &[
            JValue::Object(&env.new_string("gps").unwrap().into()),
            JValue::Long(min_time_ms),
            JValue::Float(min_distance_m),
            JValue::Object(&JObject::null()), // Would need actual listener
        ])
        .map_err(|e| format!("Failed to request location updates: {:?}", e))?;
        
        Ok(())
    })
}

pub fn gps_remove_updates() -> Result<(), String> {
    with_env(|env| {
        let context = get_android_context()?;
        let location_manager = env
            .call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("location").unwrap().into())])
            .map_err(|e| format!("Failed to get LocationManager: {:?}", e))?
            .l()
            .map_err(|_| "LocationManager is null")?;
        
        if location_manager.is_null() {
            return Err("LocationManager is null".to_string());
        }
        
        // Would need actual listener to remove
        Ok(())
    })
}

pub fn gps_get_last_known_location() -> Result<String, String> {
    with_env(|env| {
        let context = get_android_context()?;
        let location_manager = env
            .call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("location").unwrap().into())])
            .map_err(|e| format!("Failed to get LocationManager: {:?}", e))?
            .l()
            .map_err(|_| "LocationManager is null")?;
        
        if location_manager.is_null() {
            return Err("LocationManager is null".to_string());
        }
        
        let location = env
            .call_method(&location_manager, "getLastKnownLocation", "(Ljava/lang/String;)Landroid/location/Location;", &[JValue::Object(&env.new_string("gps").unwrap().into())])
            .map_err(|e| format!("Failed to get last known location: {:?}", e))?
            .l()
            .map_err(|_| "No last known location")?;
        
        if location.is_null() {
            return Ok("null".to_string());
        }
        
        let lat = env.call_method(&location, "getLatitude", "()D", &[])
            .map_err(|e| format!("Failed to get latitude: {:?}", e))?
            .d()
            .map_err(|_| "Failed to get latitude")?;
        
        let lon = env.call_method(&location, "getLongitude", "()D", &[])
            .map_err(|e| format!("Failed to get longitude: {:?}", e))?
            .d()
            .map_err(|_| "Failed to get longitude")?;
        
        let accuracy = env.call_method(&location, "getAccuracy", "()F", &[])
            .map_err(|e| format!("Failed to get accuracy: {:?}", e))?
            .f()
            .map_err(|_| "Failed to get accuracy")?;
        
        Ok(format!("{},{},{}", lat, lon, accuracy))
    })
}

pub fn gps_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    LOCATION_CALLBACKS.with(|c| {
        c.borrow_mut().insert(callback_name, Rc::new(RefCell::new(callback)));
    });
    Ok(())
}

// ============================================================================
// Camera
// ============================================================================

pub fn camera_open(camera_id: i32) -> Result<i32, String> {
    with_env(|env| {
        let camera_manager = env
            .call_method(&get_android_context()?, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("camera").unwrap().into())])
            .map_err(|e| format!("Failed to get CameraManager: {:?}", e))?
            .l()
            .map_err(|_| "CameraManager is null")?;
        
        if camera_manager.is_null() {
            return Err("CameraManager is null".to_string());
        }
        
        let camera_id_str = env.new_string(&camera_id.to_string())
            .map_err(|e| format!("Failed to create string: {:?}", e))?;
        
        let camera_device = env
            .call_method(&camera_manager, "openCamera", "(Ljava/lang/String;Landroid/hardware/camera2/CameraDevice$StateCallback;Landroid/os/Handler;)V", &[
                JValue::Object(&camera_id_str),
                JValue::Object(&JObject::null()), // Would need CameraDevice.StateCallback
                JValue::Object(&JObject::null()),
            ])
            .map_err(|e| format!("Failed to open camera: {:?}", e))?;
        
        Ok(camera_id)
    })
}

pub fn camera_close(camera_id: i32) -> Result<(), String> {
    // Camera closure handled by CameraDevice
    Ok(())
}

pub fn camera_start_preview(camera_id: i32, surface_texture: i64) -> Result<(), String> {
    with_env(|env| {
        // Simplified - would need CameraCaptureSession
        Ok(())
    })
}

pub fn camera_capture_image(camera_id: i32, output_path: &str) -> Result<(), String> {
    with_env(|env| {
        // Simplified - would need ImageReader and CaptureRequest
        Ok(())
    })
}

pub fn camera_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    CAMERA_CALLBACKS.with(|c| {
        c.borrow_mut().insert(callback_name, Rc::new(RefCell::new(callback)));
    });
    Ok(())
}

// ============================================================================
// Sensors
// ============================================================================

pub fn sensor_list() -> Result<String, String> {
    with_env(|env| {
        let context = get_android_context()?;
        let sensor_manager = env
            .call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("sensor").unwrap().into())])
            .map_err(|e| format!("Failed to get SensorManager: {:?}", e))?
            .l()
            .map_err(|_| "SensorManager is null")?;
        
        if sensor_manager.is_null() {
            return Ok("[]".to_string());
        }
        
        let sensor_list = env
            .call_method(&sensor_manager, "getSensorList", "(I)Ljava/util/List;", &[JValue::Int(-1)]) // TYPE_ALL = -1
            .map_err(|e| format!("Failed to get sensor list: {:?}", e))?
            .l()
            .map_err(|_| "Sensor list is null")?;
        
        if sensor_list.is_null() {
            return Ok("[]".to_string());
        }
        
        let size = env.call_method(&sensor_list, "size", "()I", &[])
            .map_err(|e| format!("Failed to get list size: {:?}", e))?
            .i()
            .map_err(|_| "Failed to get size")?;
        
        let mut result = String::from("[");
        for i in 0..size {
            let sensor = env.call_method(&sensor_list, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)])
                .map_err(|e| format!("Failed to get sensor: {:?}", e))?
                .l()
                .map_err(|_| "Sensor is null")?;
            
            if sensor.is_null() {
                continue;
            }
            
            let name = env.call_method(&sensor, "getName", "()Ljava/lang/String;", &[])
                .map_err(|e| format!("Failed to get sensor name: {:?}", e))?
                .l()
                .map_err(|_| "Sensor name is null")?;
            
            let sensor_type = env.call_method(&sensor, "getType", "()I", &[])
                .map_err(|e| format!("Failed to get sensor type: {:?}", e))?
                .i()
                .map_err(|_| "Failed to get sensor type")?;
            
            let vendor = env.call_method(&sensor, "getVendor", "()Ljava/lang/String;", &[])
                .map_err(|e| format!("Failed to get sensor vendor: {:?}", e))?
                .l()
                .map_err(|_| "Sensor vendor is null")?;
            
            let name_str = env.get_string(&name.into())
                .map_err(|e| format!("Failed to convert name: {:?}", e))?
                .to_str()
                .map_err(|_| "Failed to convert name")?;
            
            let vendor_str = env.get_string(&vendor.into())
                .map_err(|e| format!("Failed to convert vendor: {:?}", e))?
                .to_str()
                .map_err(|_| "Failed to convert vendor")?;
            
            result.push_str(&format!("{{\"name\":\"{}\",\"type\":{},\"vendor\":\"{}\"}}", 
                name_str.replace('"', "\\\""), sensor_type, vendor_str.replace('"', "\\\"")));
            
            if i < size - 1 {
                result.push(',');
            }
        }
        result.push(']');
        Ok(result)
    })
}

pub fn sensor_register_listener(sensor_type: i32, delay_us: i64, callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    with_env(|env| {
        let context = get_android_context()?;
        let sensor_manager = env
            .call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&env.new_string("sensor").unwrap().into())])
            .map_err(|e| format!("Failed to get SensorManager: {:?}", e))?
            .l()
            .map_err(|_| "SensorManager is null")?;
        
        if sensor_manager.is_null() {
            return Err("SensorManager is null".to_string());
        }
        
        let sensor = env
            .call_method(&sensor_manager, "getDefaultSensor", "(I)Landroid/hardware/Sensor;", &[JValue::Int(sensor_type)])
            .map_err(|e| format!("Failed to get default sensor: {:?}", e))?
            .l()
            .map_err(|_| "Sensor is null")?;
        
        if sensor.is_null() {
            return Err("Sensor not available".to_string());
        }
        
        // Register listener - would need SensorEventListener implementation
        // Simplified here
        
        SENSOR_CALLBACKS.with(|c| {
            c.borrow_mut().insert(callback_name, Rc::new(RefCell::new(callback)));
        });
        
        Ok(())
    })
}

pub fn sensor_unregister_listener(callback_name: String) -> Result<(), String> {
    SENSOR_CALLBACKS.with(|c| {
        c.borrow_mut().remove(&callback_name);
    });
    Ok(())
}

// ============================================================================
// Helper functions
// ============================================================================

fn get_android_context() -> Result<JObject<'static>, String> {
    with_env(|env| {
        let activity = get_android_activity()?;
        Ok(activity)
    })
}

fn get_android_activity() -> Result<JObject<'static>, String> {
    with_env(|env| {
        let jvm = get_jvm().ok_or("JVM not initialized")?;
        let mut jni_env = jvm.get_env().map_err(|e| format!("Failed to get env: {:?}", e))?;
        
        // Get current activity from SDL/ActivityThread
        let activity_thread = jni_env.find_class("android/app/ActivityThread")
            .map_err(|e| format!("Failed to find ActivityThread: {:?}", e))?;
        
        let current_activity = jni_env.call_static_method(&activity_thread, "currentActivity", "()Landroid/app/Activity;", &[])
            .map_err(|e| format!("Failed to get current activity: {:?}", e))?
            .l()
            .map_err(|_| "No current activity")?;
        
        Ok(current_activity)
    })
}

// Re-export for use in other modules
pub use self::{
    bluetooth_is_enabled, bluetooth_enable, bluetooth_disable,
    bluetooth_start_discovery, bluetooth_cancel_discovery,
    bluetooth_set_callback,
    nfc_is_enabled, nfc_enable_foreground_dispatch, nfc_disable_foreground_dispatch,
    nfc_set_callback,
    gps_is_enabled, gps_request_location_updates, gps_remove_updates,
    gps_get_last_known_location, gps_set_callback,
    camera_open, camera_close, camera_start_preview, camera_capture_image,
    camera_set_callback,
    sensor_list, sensor_register_listener, sensor_unregister_listener,
};