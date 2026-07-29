// Aly DirectX Backend — DirectX rendering context
// Real implementation requires Windows SDK; this provides the API shape

pub struct DirectXContext {
    pub device_name: String,
    pub initialized: bool,
    width: u32,
    height: u32,
}

impl DirectXContext {
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(DirectXContext {
            device_name: "DirectX Device".to_string(),
            initialized: true,
            width,
            height,
        })
    }

    pub fn clear(&self, _r: f32, _g: f32, _b: f32, _a: f32) -> Result<(), String> {
        if !self.initialized {
            return Err("DirectX context not initialized".to_string());
        }
        Ok(())
    }

    pub fn present(&self) -> Result<(), String> {
        if !self.initialized {
            return Err("DirectX context not initialized".to_string());
        }
        Ok(())
    }

    pub fn draw(&self) -> Result<(), String> {
        if !self.initialized {
            return Err("DirectX context not initialized".to_string());
        }
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn device_info(&self) -> String {
        format!("DirectX device: {} ({}x{})", self.device_name, self.width, self.height)
    }
}

pub fn create_context(width: u32, height: u32) -> Result<DirectXContext, String> {
    DirectXContext::new(width, height)
}

pub fn is_available() -> bool {
    cfg!(target_os = "windows")
}