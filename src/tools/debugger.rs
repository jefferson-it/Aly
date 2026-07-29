pub mod debugger {
    use crate::aly::Aly;
    
    pub fn inspect_runtime(aly: &Aly) {
        println!("--- Aly Debugger ---");
        println!("Schemas loaded: {}", aly.schemas.len());
        println!("Traits loaded: {}", aly.traits.len());
        println!("--------------------");
    }
}
pub use debugger::*;
