// iOS Native Hardware Access
// Provides Bluetooth, NFC, GPS, Camera, Sensors for iOS

#[cfg(target_os = "ios")]
use objc2::rc::Retained;
#[cfg(target_os = "ios")]
use obj2_foundation::{NSArray, NSError, NSNotificationCenter, NSObject, NSNumber, NSString};
#[cfg(target_os = "ios")]
use objc2_core_bluetooth::{
    CBCentralManager, CBCentralManagerDelegate, CBCharacteristic, CBCharacteristicProperties,
    CBManagerState, CBPeripheral, CBPeripheralDelegate, CBService, CBUUID,
};
#[cfg(target_os = "ios")]
use objc2_core_location::{
    CLLocation, CLLocationManager, CLLocationManagerDelegate, CLAuthorizationStatus,
    CLLocationAccuracy,
};
#[cfg(target_os = "ios")]
use objc2_core_nfc::{
    NFCNDEFReaderSession, NFCNDEFReaderSessionDelegate, NFCNDEFMessage, NFCNDEFPayload,
};
#[cfg(target_os = "ios")]
use objc2_av_foundation::{
    AVCaptureDevice, AVCaptureDeviceInput, AVCaptureDevicePosition, AVCaptureSession,
    AVCaptureSessionPreset, AVCapturePhotoOutput, AVCapturePhotoSettings,
    AVCaptureVideoPreviewLayer, AVMediaType,
};
#[cfg(target_os = "ios")]
use objc2_core_motion::{
    CMMotionManager, CMDeviceMotion, CMAccelerometerData, CMGyroData, CMMagnetometerData,
};

#[cfg(target_os = "ios")]
use std::cell::RefCell;
#[cfg(target_os = "ios")]
use std::rc::Rc;
#[cfg(target_os = "ios")]
use std::collections::HashMap;

#[cfg(target_os = "ios")]
thread_local! {
    static BLUETOOTH_MANAGER: RefCell<Option<Retained<CBCentralManager>>> = RefCell::new(None);
    static BLUETOOTH_PERIPHERALS: RefCell<HashMap<String, Retained<CBPeripheral>>> = RefCell::new(HashMap::new());
    static BLUETOOTH_CALLBACKS: RefCell<HashMap<String, Box<dyn Fn(String) + Send + Sync>>> = RefCell::new(HashMap::new());
    
    static NFC_SESSION: RefCell<Option<Retained<NFCNDEFReaderSession>>> = RefCell::new(None);
    static NFC_CALLBACKS: RefCell<HashMap<String, Box<dyn Fn(String) + Send + Sync>>> = RefCell::new(HashMap::new());
    
    static LOCATION_MANAGER: RefCell<Option<Retained<CLLocationManager>>> = RefCell::new(None);
    static LOCATION_CALLBACKS: RefCell<HashMap<String, Box<dyn Fn(String) + Send + Sync>>> = RefCell::new(HashMap::new());
    
    static CAMERA_SESSION: RefCell<Option<Retained<AVCaptureSession>>> = RefCell::new(None);
    static CAMERA_CALLBACKS: RefCell<HashMap<String, Box<dyn Fn(String) + Send + Sync>>> = RefCell::new(HashMap::new());
    
    static SENSOR_MANAGER: RefCell<Option<Retained<CMMotionManager>>> = RefCell::new(None);
    static SENSOR_CALLBACKS: RefCell<HashMap<String, Box<dyn Fn(String) + Send + Sync>>> = RefCell::new(HashMap::new());
}

// ============================================================================
// Bluetooth Low Energy (CoreBluetooth)
// ============================================================================

#[cfg(target_os = "ios")]
pub fn bluetooth_init() -> Result<(), String> {
    use objc2_core_bluetooth::CBCentralManager;
    use objc2_core_bluetooth::CBCentralManagerDelegate;
    use objc2_foundation::NSObject;
    
    let manager = CBCentralManager::new();
    BLUETOOTH_MANAGER.with(|m| *m.borrow_mut() = Some(manager));
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn bluetooth_is_enabled() -> bool {
    use objc2_core_bluetooth::CBManagerState;
    
    BLUETOOTH_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.state() == CBManagerState::PoweredOn
        } else {
            false
        }
    })
}

#[cfg(target_os = "ios")]
pub fn bluetooth_start_scan(service_uuids: Option<Vec<String>>, callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    BLUETOOTH_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    
    BLUETOOTH_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            let uuids = if let Some(uuids) = service_uuids {
                let ns_uuids: Vec<Retained<CBUUID>> = uuids.iter()
                    .map(|u| CBUUID::stringWithString(NSString::from_str(u)))
                    .collect();
                Some(NSArray::from_vec(ns_uuids))
            } else {
                None
            };
            
            manager.scanForPeripheralsWithServices_options(&uuids, &std::collections::HashMap::new());
            Ok(())
        } else {
            Err("Bluetooth manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn bluetooth_stop_scan() -> Result<(), String> {
    BLUETOOTH_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopScan();
            Ok(())
        } else {
            Err("Bluetooth manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn bluetooth_connect(peripheral_id: String) -> Result<(), String> {
    BLUETOOTH_PERIPHERALS.with(|p| {
        if let Some(peripheral) = p.borrow().get(&peripheral_id) {
            BLUETOOTH_MANAGER.with(|m| {
                if let Some(manager) = m.borrow().as_ref() {
                    manager.connectPeripheral_options(peripheral, &std::collections::HashMap::new());
                    Ok(())
                } else {
                    Err("Bluetooth manager not initialized".to_string())
                }
            })
        } else {
            Err("Peripheral not found".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn bluetooth_disconnect(peripheral_id: String) -> Result<(), String> {
    BLUETOOTH_PERIPHERALS.with(|p| {
        if let Some(peripheral) = p.borrow().get(&peripheral_id) {
            BLUETOOTH_MANAGER.with(|m| {
                if let Some(manager) = m.borrow().as_ref() {
                    manager.cancelPeripheralConnection(peripheral);
                    Ok(())
                } else {
                    Err("Bluetooth manager not initialized".to_string())
                }
            })
        } else {
            Err("Peripheral not found".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn bluetooth_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    BLUETOOTH_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    Ok(())
}

// ============================================================================
// NFC (CoreNFC)
// ============================================================================

#[cfg(target_os = "ios")]
pub fn nfc_is_supported() -> bool {
    use objc2_core_nfc::NFCNDEFReaderSession;
    NFCNDEFReaderSession::readingAvailable()
}

#[cfg(target_os = "ios")]
pub fn nfc_begin_session(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    use objc2_core_nfc::{NFCNDEFReaderSession, NFCNDEFReaderSessionDelegate};
    use objc2_foundation::NSObject;
    
    NFC_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    
    let session = NFCNDEFReaderSession::initWithDelegate_queue_delegate(
        NFCNDEFReaderSession::alloc(),
        None, // main queue
        None, // delegate - would need custom implementation
    );
    
    NFC_SESSION.with(|s| *s.borrow_mut() = Some(session));
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn nfc_invalidate_session() -> Result<(), String> {
    NFC_SESSION.with(|s| {
        if let Some(session) = s.borrow().as_ref() {
            session.invalidate();
            *s.borrow_mut() = None;
            Ok(())
        } else {
            Err("No active NFC session".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn nfc_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    NFC_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    Ok(())
}

// ============================================================================
// GPS / Location (CoreLocation)
// ============================================================================

#[cfg(target_os = "ios")]
pub fn gps_init() -> Result<(), String> {
    use objc2_core_location::CLLocationManager;
    
    let manager = CLLocationManager::new();
    manager.setDesiredAccuracy(CLLocationAccuracy::Best);
    manager.setDistanceFilter(10.0); // 10 meters
    
    LOCATION_MANAGER.with(|m| *m.borrow_mut() = Some(manager));
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn gps_request_authorization(when_in_use: bool) -> Result<(), String> {
    LOCATION_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            if when_in_use {
                manager.requestWhenInUseAuthorization();
            } else {
                manager.requestAlwaysAuthorization();
            }
            Ok(())
        } else {
            Err("Location manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn gps_start_updates() -> Result<(), String> {
    LOCATION_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.startUpdatingLocation();
            Ok(())
        } else {
            Err("Location manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn gps_stop_updates() -> Result<(), String> {
    LOCATION_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopUpdatingLocation();
            Ok(())
        } else {
            Err("Location manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn gps_get_last_location() -> Result<String, String> {
    LOCATION_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            if let Some(location) = manager.location() {
                let lat = location.coordinate().latitude;
                let lon = location.coordinate().longitude;
                let accuracy = location.horizontalAccuracy();
                Ok(format!("{},{},{}", lat, lon, accuracy))
            } else {
                Err("No location available".to_string())
            }
        } else {
            Err("Location manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn gps_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    LOCATION_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    Ok(())
}

// ============================================================================
// Camera (AVFoundation)
// ============================================================================

#[cfg(target_os = "ios")]
pub fn camera_open(position: &str) -> Result<i32, String> {
    use objc2_av_foundation::{AVCaptureDevice, AVCaptureDevicePosition};
    
    let position_enum = match position {
        "front" => AVCaptureDevicePosition::Front,
        "back" => AVCaptureDevicePosition::Back,
        _ => AVCaptureDevicePosition::Back,
    };
    
    let device = AVCaptureDevice::defaultDeviceWithMediaType(AVMediaType::Video)
        .ok_or("No camera available")?;
    
    let input = AVCaptureDeviceInput::deviceInputWithDevice_error(&device)
        .map_err(|e| format!("Failed to create camera input: {:?}", e))?;
    
    let session = AVCaptureSession::new();
    session.setSessionPreset(AVCaptureSessionPreset::High);
    
    if session.canAddInput(&input) {
        session.addInput(&input);
    } else {
        return Err("Cannot add camera input".to_string());
    }
    
    let output = objc2_av_foundation::AVCapturePhotoOutput::new();
    if session.canAddOutput(&output) {
        session.addOutput(&output);
    }
    
    session.startRunning();
    
    CAMERA_SESSION.with(|s| *s.borrow_mut() = Some(session));
    
    Ok(0) // Camera ID
}

#[cfg(target_os = "ios")]
pub fn camera_close(camera_id: i32) -> Result<(), String> {
    CAMERA_SESSION.with(|s| {
        if let Some(session) = s.borrow().as_ref() {
            session.stopRunning();
        }
        *s.borrow_mut() = None;
    });
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn camera_capture_photo(camera_id: i32, output_path: &str) -> Result<(), String> {
    use objc2_av_foundation::AVCapturePhotoOutput;
    use objc2_av_foundation::AVCapturePhotoSettings;
    
    CAMERA_SESSION.with(|s| {
        if let Some(session) = s.borrow().as_ref() {
            let output = AVCapturePhotoOutput::new();
            let settings = AVCapturePhotoSettings::new();
            
            // Would need delegate to capture photo
            // Simplified here
            Ok(())
        } else {
            Err("Camera not open".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn camera_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    CAMERA_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    Ok(())
}

// ============================================================================
// Sensors (CoreMotion)
// ============================================================================

#[cfg(target_os = "ios")]
pub fn sensor_init() -> Result<(), String> {
    use objc2_core_motion::CMMotionManager;
    
    let manager = CMMotionManager::new();
    SENSOR_MANAGER.with(|m| *m.borrow_mut() = Some(manager));
    Ok(())
}

#[cfg(target_os = "ios")]
pub fn sensor_start_accelerometer(delay_ms: u64) -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.setAccelerometerUpdateInterval(delay_ms as f64 / 1000.0);
            manager.startAccelerometerUpdatesToQueue_withHandler(
                None, // main queue
                Box::new(|data, _error| {
                    if let Some(accel) = data {
                        let x = accel.acceleration().x;
                        let y = accel.acceleration().y;
                        let z = accel.acceleration().z;
                        
                        SENSOR_CALLBACKS.with(|c| {
                            let callbacks = c.borrow();
                            for callback in callbacks.values() {
                                callback(format!("accelerometer,{},{},{}", x, y, z));
                            }
                        });
                    }
                },
            );
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_stop_accelerometer() -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopAccelerometerUpdates();
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_start_gyroscope(delay_ms: u64) -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.setGyroUpdateInterval(delay_ms as f64 / 1000.0);
            manager.startGyroUpdatesToQueue_withHandler(
                None,
                Box::new(|data, _error| {
                    if let Some(gyro) = data {
                        let x = gyro.rotationRate().x;
                        let y = gyro.rotationRate().y;
                        let z = gyro.rotationRate().z;
                        
                        SENSOR_CALLBACKS.with(|c| {
                            let callbacks = c.borrow();
                            for callback in callbacks.values() {
                                callback(format!("gyroscope,{},{},{}", x, y, z));
                            }
                        });
                    }
                },
            );
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_stop_gyroscope() -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopGyroUpdates();
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_start_magnetometer(delay_ms: u64) -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.setMagnetometerUpdateInterval(delay_ms as f64 / 1000.0);
            manager.startMagnetometerUpdatesToQueue_withHandler(
                None,
                Box::new(|data, _error| {
                    if let Some(mag) = data {
                        let x = mag.magneticField().x;
                        let y = mag.magneticField().y;
                        let z = mag.magneticField().z;
                        
                        SENSOR_CALLBACKS.with(|c| {
                            let callbacks = c.borrow();
                            for callback in callbacks.values() {
                                callback(format!("magnetometer,{},{},{}", x, y, z));
                            }
                        });
                    }
                },
            );
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_stop_magnetometer() -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopMagnetometerUpdates();
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_start_device_motion(delay_ms: u64) -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.setDeviceMotionUpdateInterval(delay_ms as f64 / 1000.0);
            manager.startDeviceMotionUpdatesToQueue_withHandler(
                None,
                Box::new(|data, _error| {
                    if let Some(motion) = data {
                        let attitude = motion.attitude();
                        let pitch = attitude.pitch();
                        let roll = attitude.roll();
                        let yaw = attitude.yaw();
                        
                        let rotation_rate = motion.rotationRate();
                        let rot_x = rotation_rate.x;
                        let rot_y = rotation_rate.y;
                        let rot_z = rotation_rate.z;
                        
                        let gravity = motion.gravity();
                        let g_x = gravity.x;
                        let g_y = gravity.y;
                        let g_z = gravity.z;
                        
                        let user_accel = motion.userAcceleration();
                        let ua_x = user_accel.x;
                        let ua_y = user_accel.y;
                        let ua_z = user_accel.z;
                        
                        SENSOR_CALLBACKS.with(|c| {
                            let callbacks = c.borrow();
                            for callback in callbacks.values() {
                                callback(format!(
                                    "devicemotion,{},{},{},{},{},{},{},{},{},{},{}", 
                                    pitch, roll, yaw, rot_x, rot_y, rot_z, g_x, g_y, g_z, ua_x, ua_y, ua_z
                                ));
                            }
                        });
                    }
                },
            );
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_stop_device_motion() -> Result<(), String> {
    SENSOR_MANAGER.with(|m| {
        if let Some(manager) = m.borrow().as_ref() {
            manager.stopDeviceMotionUpdates();
            Ok(())
        } else {
            Err("Motion manager not initialized".to_string())
        }
    })
}

#[cfg(target_os = "ios")]
pub fn sensor_set_callback(callback_name: String, callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> {
    SENSOR_CALLBACKS.with(|c| c.borrow_mut().insert(callback_name, callback));
    Ok(())
}

// ============================================================================
// Stub implementations for non-iOS targets
// ============================================================================

#[cfg(not(target_os = "ios"))]
pub fn bluetooth_init() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_is_enabled() -> bool { false }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_start_scan(_service_uuids: Option<Vec<String>>, _callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_stop_scan() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_connect(_peripheral_id: String) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_disconnect(_peripheral_id: String) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn bluetooth_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }

#[cfg(not(target_os = "ios"))]
pub fn nfc_is_supported() -> bool { false }
#[cfg(not(target_os = "ios"))]
pub fn nfc_begin_session(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn nfc_invalidate_session() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn nfc_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }

#[cfg(not(target_os = "ios"))]
pub fn gps_init() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn gps_request_authorization(_when_in_use: bool) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn gps_start_updates() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn gps_stop_updates() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn gps_get_last_location() -> Result<String, String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn gps_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }

#[cfg(not(target_os = "ios"))]
pub fn camera_open(_position: &str) -> Result<i32, String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn camera_close(_camera_id: i32) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn camera_capture_photo(_camera_id: i32, _output_path: &str) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn camera_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }

#[cfg(not(target_os = "ios"))]
pub fn sensor_init() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_start_accelerometer(_delay_ms: u64) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_stop_accelerometer() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_start_gyroscope(_delay_ms: u64) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_stop_gyroscope() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_start_magnetometer(_delay_ms: u64) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_stop_magnetometer() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_start_device_motion(_delay_ms: u64) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_stop_device_motion() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn sensor_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }

#[cfg(not(target_os = "ios"))]
pub fn nfc_is_supported() -> bool { false }
#[cfg(not(target_os = "ios"))]
pub fn nfc_begin_session(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn nfc_invalidate_session() -> Result<(), String> { Err("iOS only".to_string()) }
#[cfg(not(target_os = "ios"))]
pub fn nfc_set_callback(_callback_name: String, _callback: Box<dyn Fn(String) + Send + Sync>) -> Result<(), String> { Err("iOS only".to_string()) }