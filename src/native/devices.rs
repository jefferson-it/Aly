mod devices {
    
    use crate::native::types::ValueData;
    use crate::native::create_object::Object;
    use linked_hash_map::LinkedHashMap;

    #[derive(Clone, PartialEq)]
    pub struct DeviceInstance {
        pub vendor: String,
        pub manufacture: String,
        pub size: String,
        pub device_type: String,
        pub port: String,
        pub is_mounted: bool,
        pub mount_path: String,
        pub health: String,
        pub temperature: i32,
    }

    impl DeviceInstance {
        pub fn new(device_type: &str, port: &str) -> Self {
            DeviceInstance {
                vendor: "Generic".to_string(),
                manufacture: "Standard Hardware".to_string(),
                size: "512GB".to_string(),
                device_type: device_type.to_string(),
                port: port.to_string(),
                is_mounted: false,
                mount_path: String::new(),
                health: "GOOD".to_string(),
                temperature: 38,
            }
        }

        pub fn info(&self) -> ValueData {
            let mut map = LinkedHashMap::new();
            map.insert("vendor".to_string(), ValueData::String(self.vendor.clone()));
            map.insert("manufacture".to_string(), ValueData::String(self.manufacture.clone()));
            map.insert("size".to_string(), ValueData::String(self.size.clone()));
            map.insert("device_type".to_string(), ValueData::String(self.device_type.clone()));
            map.insert("port".to_string(), ValueData::String(self.port.clone()));
            map.insert("is_mounted".to_string(), ValueData::Bool(self.is_mounted));
            ValueData::Object(Object::from_map(map))
        }

        pub fn mount(&mut self, path: Option<String>) -> Result<ValueData, String> {
            self.is_mounted = true;
            self.mount_path = path.unwrap_or_else(|| "/mnt/dev".to_string());
            Ok(ValueData::Bool(true))
        }

        pub fn umount(&mut self) -> Result<ValueData, String> {
            if !self.is_mounted {
                return Err("DeviceUnmountError: Dispositivo não está montado.".to_string());
            }
            self.is_mounted = false;
            self.mount_path.clear();
            Ok(ValueData::Bool(true))
        }

        pub fn status(&self) -> ValueData {
            let mut map = LinkedHashMap::new();
            map.insert("health".to_string(), ValueData::String(self.health.clone()));
            map.insert("temperature".to_string(), ValueData::Int(self.temperature));
            ValueData::Object(Object::from_map(map))
        }
    }

    pub fn get_device(dev_type: &str, index: usize) -> ValueData {
        // Return -1 if hardware type is unsupported or out of index
        let supported_types = vec!["SATA", "USB", "MONITOR", "PCI", "NVMe"];
        if !supported_types.contains(&dev_type) {
            return ValueData::Int(-1);
        }

        let dev = DeviceInstance::new(dev_type, &format!("port_{}", index));
        let mut obj_map = LinkedHashMap::new();
        obj_map.insert("vendor".to_string(), ValueData::String(dev.vendor));
        obj_map.insert("manufacture".to_string(), ValueData::String(dev.manufacture));
        obj_map.insert("size".to_string(), ValueData::String(dev.size));
        obj_map.insert("device_type".to_string(), ValueData::String(dev.device_type));
        obj_map.insert("port".to_string(), ValueData::String(dev.port));
        obj_map.insert("is_mounted".to_string(), ValueData::Bool(dev.is_mounted));
        
        ValueData::Object(Object::from_map(obj_map))
    }

    pub fn create_devices_global() -> ValueData {
        let mut root_map = LinkedHashMap::new();
        let types = vec!["SATA", "USB", "MONITOR", "PCI", "NVMe"];

        for t in types {
            let mut handler_map = LinkedHashMap::new();
            handler_map.insert("type".to_string(), ValueData::String(t.to_string()));
            root_map.insert(t.to_string(), ValueData::Object(Object::from_map(handler_map)));
        }

        ValueData::Object(Object::from_map(root_map))
    }
}

pub use devices::*;
