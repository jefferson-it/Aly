#[cfg(feature = "android")]
pub mod android_template;
#[cfg(feature = "android")]
pub mod android_build;

pub mod fmt;
pub mod linter;
pub mod doc;
pub mod test_runner;
pub mod debugger;
pub mod lsp;
pub mod hotreload;
pub mod jni_gen;
