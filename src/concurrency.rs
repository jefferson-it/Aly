pub mod concurrency {
    use std::sync::{Arc, RwLock};
    use crate::native::types::ValueData;

    pub struct AtomicVar {
        pub value: Arc<RwLock<ValueData>>,
    }

    impl AtomicVar {
        pub fn new(value: ValueData) -> Self {
            AtomicVar {
                value: Arc::new(RwLock::new(value)),
            }
        }
    }
}
pub use concurrency::*;
