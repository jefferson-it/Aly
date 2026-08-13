/// Aly runtime — owns the execution context for a single Aly program.
///
/// # Architecture decision: global runtime
///
/// The current architecture passes the runtime through a module-level accessor.
/// This is kept for backwards-compatibility but the `unsafe` static has been
/// replaced with a `thread_local!` + `RefCell` pattern.
///
/// Rationale for `thread_local!`:
/// - Zero `unsafe` code in the public API.
/// - Naturally isolated per OS thread, which matches the current single-threaded
///   execution model.
/// - Cheap — no heap allocation, no mutex overhead.
/// - When concurrent execution is added in the future, each worker thread will
///   own its own runtime cell, which is exactly what we want for task isolation.
/// - Alternative considered: `Arc<Mutex<Aly>>` — unnecessarily complex and
///   slower for a single-threaded interpreter.
///
/// The public `get_runtime()` function still returns `&'static mut Aly`-shaped
/// access via a raw pointer so that callers do not need to hold a `RefMut`
/// guard across function boundaries.  This is sound inside a single thread.

mod aly {
    use std::cell::RefCell;
    use std::env;

    use std::collections::HashMap;
    use crate::schema::SchemaDef;
    use crate::trait_system::TraitDef;
    use crate::module_system::Module;
    use crate::native::create_object::Object;
    

    use crate::{
        error::{AlyError, AlyResult},
        lexer::Lexer,
        native::{
            fs::read_file, fun_input, fun_print, fun_drop, fun_addr, fun_deref, fun_store,
            fun_alloc, fun_alloc_typed, fun_free, fun_is_null, fun_ptr_type, process_value, tomb,
            types::{coerce, parse_type_annotation, Type, Validator, ValueData},
            vars::*,
            http_api::http_api_serve,
        },
        plugin::call_plugin,
        runtime::interpreter::exec,
        runtime::parser::get_lexer,
        tokens::Tokens,
        validators::{
            str::remove_quoted_str,
            structures::{is_close, is_opened},
        },
    };

    /// Action types for the Aly runtime
    #[derive(Clone, Debug)]
    pub enum Act {
        Run,
        Cli,
        Comp,
    }

    // ──────────────────────────────────────────────────────────────────────────
    // The runtime struct
    // ──────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ControlFlow {
    None,
    Break,
    Continue,
}

#[derive(Clone)]
    pub struct Aly {
        pub action: Option<Act>,
        pub datas: Vec<Var>,
        pub control_flow: ControlFlow,
        pub schemas: Vec<SchemaDef>,
        pub traits: Vec<TraitDef>,
        pub type_aliases: HashMap<String, Type>,
        pub strict_mode: bool,
        pub modules: HashMap<String, Module>,
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Thread-local storage — replaces `static mut RUNTIME`
    // ──────────────────────────────────────────────────────────────────────────

    thread_local! {
    static RUNTIME: RefCell<Aly> = RefCell::new(Aly {
            action: None,
            datas: Vec::new(),
            control_flow: ControlFlow::None,
            schemas: vec![],
            traits: vec![],
            type_aliases: HashMap::new(),
            strict_mode: false,
            modules: HashMap::new(),
        });
    }



    /// Get a raw mutable pointer to the thread-local runtime.
    ///
    /// # Safety
    /// This is safe when called from a single thread (the current execution
    /// model).  The pointer is valid for the duration of the thread-local's
    /// lifetime.  If multi-threaded execution is added in the future this
    /// function must be replaced with an `Arc<Mutex<Aly>>` accessor.
    pub fn get_runtime() -> &'static mut Aly {
        RUNTIME.with(|cell| {
            // SAFETY: single-threaded interpreter; the borrow is released
            // before any recursive call could reach here.
            unsafe { &mut *cell.as_ptr() }
        })
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Aly implementation
    // ──────────────────────────────────────────────────────────────────────────

    impl Aly {
        pub fn def_action(&mut self, act: Act) -> AlyResult<()> {
            if self.action.is_some() {
                return Err(AlyError::runtime("A ação já foi definida e não pode ser alterada."));
            }
            self.action = Some(act);
            Ok(())
        }

        pub fn new() -> AlyResult<Aly> {
            let _cwd = env::current_dir().map_err(|e| {
                AlyError::runtime(format!(
                    "Erro ao iniciar o programa: não foi possível obter o diretório atual. {}",
                    e
                ))
            })?;

Ok(Aly {
        action: None,
        control_flow: ControlFlow::None,
        datas: vec![Var::new(
            "this".to_string(),
            ValueData::String("None".to_string()),
            true,
        )],
        schemas: vec![],
        traits: vec![],
        type_aliases: HashMap::new(),
        strict_mode: false,
        modules: HashMap::new(),
    })

        }

        // ── Runtime ──────────────────────────────────────────────────────────

        pub fn run(&mut self, file: String) {
            self.datas.push(Var::new(
                String::from("print"),
                fun_print as fn(String) -> Box<dyn Validator>,
                false,
            ));
            self.datas.push(Var::new(
                String::from("input"),
                fun_input as fn(String) -> Box<dyn Validator>,
                false,
            ));
            self.datas.push(Var::new(
                String::from("tomb"),
                tomb as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("addr"),
                fun_addr as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("deref"),
                fun_deref as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("store"),
                fun_store as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("alloc"),
                fun_alloc as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("alloc_typed"),
                fun_alloc_typed as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("free"),
                fun_free as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("is_null"),
                fun_is_null as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("ptr_type"),
                fun_ptr_type as fn(&[ValueData]) -> ValueData,
                false,
            ));
            self.datas.push(Var::new(
                String::from("drop"),
                fun_drop as fn(String) -> Box<dyn Validator>,
                false,
            ));
            self.datas.push(Var::new(
                String::from("devices"),
                crate::native::devices::create_devices_global(),
                false,
            ));
            #[cfg(feature = "gui")]
            {
                self.datas.push(Var::new(
                    String::from("gtk_installer_app"),
                    crate::native::gtk::gtk_installer_app as fn(String) -> Box<dyn Validator>,
                    false,
                ));
                self.datas.push(Var::new(
                    String::from("gtk_installer"),
                    crate::native::gtk::gtk_installer_app as fn(String) -> Box<dyn Validator>,
                    false,
                ));
            }

            // OS functions
            use crate::native::os::*;
            self.datas.push(Var::new(String::from("os_platform"), os_platform as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.platform"), os_platform as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_arch"), os_arch as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.arch"), os_arch as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_type"), os_type as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.type"), os_type as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_release"), os_release as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.release"), os_release as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_hostname"), os_hostname as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.hostname"), os_hostname as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_homedir"), os_homedir as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.homedir"), os_homedir as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_tmpdir"), os_tmpdir as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.tmpdir"), os_tmpdir as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_pid"), os_pid as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.pid"), os_pid as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_ppid"), os_ppid as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.ppid"), os_ppid as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_endianness"), os_endianness as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.endianness"), os_endianness as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_eol"), os_eol as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.eol"), os_eol as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_uptime"), os_uptime as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.uptime"), os_uptime as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_totalmem"), os_totalmem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.totalmem"), os_totalmem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_freemem"), os_freemem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.freemem"), os_freemem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_usedmem"), os_usedmem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.usedmem"), os_usedmem as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_loadavg"), os_loadavg as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.loadavg"), os_loadavg as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_cpus"), os_cpus as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.cpus"), os_cpus as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_network_interfaces"), os_network_interfaces as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.network_interfaces"), os_network_interfaces as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_userinfo"), os_userinfo as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.userinfo"), os_userinfo as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_syscall"), os_syscall as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.syscall"), os_syscall as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os_exec"), os_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("os.exec"), os_exec as fn(String) -> Box<dyn Validator>, false));

            // System functions
            use crate::native::std::*;
            self.datas.push(Var::new(String::from("sys_cwd"), sys_cwd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys.cwd"), sys_cwd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys_env"), sys_env as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys.env"), sys_env as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys_platform"), sys_platform as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys.platform"), sys_platform as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys_exit"), sys_exit as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sys.exit"), sys_exit as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("sys_args"), sys_args as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("sys.args"), sys_args as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("sys_hostname"), sys_hostname as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("sys.hostname"), sys_hostname as fn(String) -> Box<dyn Validator>, false));

// Math functions
self.datas.push(Var::new(String::from("math_abs"), math_abs as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.abs"), math_abs as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math_max"), math_max as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.max"), math_max as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math_min"), math_min as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.min"), math_min as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math_round"), math_round as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.round"), math_round as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math_floor"), math_floor as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.floor"), math_floor as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math_ceil"), math_ceil as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("math.ceil"), math_ceil as fn(String) -> Box<dyn Validator>, false));

// FS functions
            self.datas.push(Var::new(String::from("fs_read"), fs_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs.read"), fs_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs_write"), fs_write as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs.write"), fs_write as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs_append"), fs_append as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs.append"), fs_append as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs_exists"), fs_exists as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs.exists"), fs_exists as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs_remove"), fs_remove as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("fs.remove"), fs_remove as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs_mkdir"), fs_mkdir as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs.mkdir"), fs_mkdir as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs_home"), fs_home as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs.home"), fs_home as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs_is_file"), fs_is_file as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs.is_file"), fs_is_file as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs_touch"), fs_touch as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("fs.touch"), fs_touch as fn(String) -> Box<dyn Validator>, false));

// String functions
self.datas.push(Var::new(String::from("str_upper"), str_upper as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.upper"), str_upper as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str_lower"), str_lower as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.lower"), str_lower as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str_trim"), str_trim as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.trim"), str_trim as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str_contains"), str_contains as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.contains"), str_contains as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str_replace"), str_replace as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.replace"), str_replace as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str_split"), str_split as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("str.split"), str_split as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_starts_with"), str_starts_with as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.starts_with"), str_starts_with as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_ends_with"), str_ends_with as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.ends_with"), str_ends_with as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_reverse"), str_reverse as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.reverse"), str_reverse as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("vec_join"), vec_join as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("vec.join"), vec_join as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_pad"), str_pad as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.pad"), str_pad as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_lstrip"), str_lstrip as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.lstrip"), str_lstrip as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str_rstrip"), str_rstrip as fn(String) -> Box<dyn Validator>, false));
self.datas.push(Var::new(String::from("str.rstrip"), str_rstrip as fn(String) -> Box<dyn Validator>, false));

// Console functions
            use crate::native::console::*;
            self.datas.push(Var::new(String::from("console_log"), console_log as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console.log"), console_log as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console_warn"), console_warn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console.warn"), console_warn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console_error"), console_error as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console.error"), console_error as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console_success"), console_success as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("console.success"), console_success as fn(String) -> Box<dyn Validator>, false));

            // GUI functions
            #[cfg(feature = "gui")]
            {
                use crate::native::gui_abstraction::*;
                self.datas.push(Var::new(String::from("gui.useBackend"), gui_use_backend as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.window"), gui_window as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.button"), gui_button as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.label"), gui_label as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.div"), gui_div as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.input"), gui_input as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.textarea"), gui_textarea as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.password"), gui_password as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.checkbox"), gui_checkbox as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.radio"), gui_radio as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.slider"), gui_slider as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.progressbar"), gui_progressbar as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.dropdown"), gui_dropdown as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.spinner"), gui_spinner as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.container"), gui_container as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.insert"), gui_insert as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.set"), gui_set as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.get"), gui_get as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.on"), gui_on as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gui.run"), gui_run as fn(String) -> Box<dyn Validator>, false));
            }

            // IoT functions
            #[cfg(feature = "mqtt")]
            {
                use crate::native::mqtt::*;
                self.datas.push(Var::new(String::from("mqtt_connect"), mqtt_connect as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt.connect"), mqtt_connect as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt_publish"), mqtt_publish as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt.publish"), mqtt_publish as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt_subscribe"), mqtt_subscribe as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt.subscribe"), mqtt_subscribe as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt_unsubscribe"), mqtt_unsubscribe as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt.unsubscribe"), mqtt_unsubscribe as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt_disconnect"), mqtt_disconnect as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("mqtt.disconnect"), mqtt_disconnect as fn(String) -> Box<dyn Validator>, false));
            }

            #[cfg(feature = "serial")]
            {
                use crate::native::serial::*;
                self.datas.push(Var::new(String::from("serial_list"), serial_list as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.list"), serial_list as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_open"), serial_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.open"), serial_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_close"), serial_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.close"), serial_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_write"), serial_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.write"), serial_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_read"), serial_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.read"), serial_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_read_line"), serial_read_line as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.read_line"), serial_read_line as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial_on_data"), serial_on_data as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("serial.on_data"), serial_on_data as fn(String) -> Box<dyn Validator>, false));
            }

            #[cfg(feature = "rpi-gpio")]
            {
                use crate::native::gpio::*;
                self.datas.push(Var::new(String::from("gpio_setup"), gpio_setup as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.setup"), gpio_setup as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_write"), gpio_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.write"), gpio_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_read"), gpio_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.read"), gpio_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_on_rising"), gpio_on_rising as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.on_rising"), gpio_on_rising as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_on_falling"), gpio_on_falling as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.on_falling"), gpio_on_falling as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_on_change"), gpio_on_change as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.on_change"), gpio_on_change as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_remove_callback"), gpio_remove_callback as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.remove_callback"), gpio_remove_callback as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_pwm"), gpio_pwm as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.pwm"), gpio_pwm as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio_cleanup"), gpio_cleanup as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("gpio.cleanup"), gpio_cleanup as fn(String) -> Box<dyn Validator>, false));
            }

            #[cfg(feature = "rpi-i2c")]
            {
                use crate::native::i2c::*;
                self.datas.push(Var::new(String::from("i2c_open"), i2c_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.open"), i2c_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c_write"), i2c_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.write"), i2c_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c_read"), i2c_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.read"), i2c_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c_write_read"), i2c_write_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.write_read"), i2c_write_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c_close"), i2c_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.close"), i2c_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c_scan"), i2c_scan as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("i2c.scan"), i2c_scan as fn(String) -> Box<dyn Validator>, false));
            }

            #[cfg(feature = "rpi-spi")]
            {
                use crate::native::spi::*;
                self.datas.push(Var::new(String::from("spi_open"), spi_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.open"), spi_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi_transfer"), spi_transfer as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.transfer"), spi_transfer as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi_write"), spi_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.write"), spi_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi_read"), spi_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.read"), spi_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi_close"), spi_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.close"), spi_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi_set_mode"), spi_set_mode as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("spi.set_mode"), spi_set_mode as fn(String) -> Box<dyn Validator>, false));
            }

            #[cfg(feature = "rpi-uart")]
            {
                use crate::native::uart::*;
                self.datas.push(Var::new(String::from("uart_open"), uart_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart.open"), uart_open as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart_write"), uart_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart.write"), uart_write as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart_read"), uart_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart.read"), uart_read as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart_close"), uart_close as fn(String) -> Box<dyn Validator>, false));
                self.datas.push(Var::new(String::from("uart.close"), uart_close as fn(String) -> Box<dyn Validator>, false));
            }

            // Crypto functions
            use crate::native::crypto::*;
            self.datas.push(Var::new(String::from("crypto_md5"), crypto_md5 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.md5"), crypto_md5 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_sha256"), crypto_sha256 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.sha256"), crypto_sha256 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_sha512"), crypto_sha512 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.sha512"), crypto_sha512 as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_hmac"), crypto_hmac as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.hmac"), crypto_hmac as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_argon2_hash"), crypto_argon2_hash as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.argon2_hash"), crypto_argon2_hash as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_argon2_verify"), crypto_argon2_verify as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.argon2_verify"), crypto_argon2_verify as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_bcrypt_hash"), crypto_bcrypt_hash as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.bcrypt_hash"), crypto_bcrypt_hash as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto_bcrypt_verify"), crypto_bcrypt_verify as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("crypto.bcrypt_verify"), crypto_bcrypt_verify as fn(String) -> Box<dyn Validator>, false));

            // HTTP client - curl object (fetch-style API)
            use crate::native::curl::create_curl_object;
            let curl_obj = create_curl_object();
            if let ValueData::Object(obj) = curl_obj.valid().1 {
                self.datas.push(Var::new(String::from("curl"), obj.clone(), false));
            }

            // HTTP API server
            self.datas.push(Var::new(String::from("http_api_serve"), http_api_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("http_api.serve"), http_api_serve as fn(String) -> Box<dyn Validator>, false));

            // Shell functions
            use crate::native::shell::*;
            self.datas.push(Var::new(String::from("shell_exec"), shell_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.exec"), shell_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_exec_lines"), shell_exec_lines as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.exec_lines"), shell_exec_lines as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_exec_status"), shell_exec_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.exec_status"), shell_exec_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_spawn"), shell_spawn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.spawn"), shell_spawn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_wait"), shell_wait as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.wait"), shell_wait as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_wait_output"), shell_wait_output as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.wait_output"), shell_wait_output as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_kill"), shell_kill as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.kill"), shell_kill as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_pid_exists"), shell_pid_exists as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.pid_exists"), shell_pid_exists as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_pipe"), shell_pipe as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.pipe"), shell_pipe as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_pipe_lines"), shell_pipe_lines as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.pipe_lines"), shell_pipe_lines as fn(String) -> Box<dyn Validator>, false));

            // GraphQL
            use crate::native::graphql::{graphql_register_schema, graphql_introspect, graphql_query, graphql_serve};
            self.datas.push(Var::new(String::from("graphql_register_schema"), graphql_register_schema as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("graphql.introspect"), graphql_introspect as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("graphql_query"), graphql_query as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("graphql.serve"), graphql_serve as fn(String) -> Box<dyn Validator>, false));

            // gRPC
            use crate::native::grpc::{grpc_register_service, grpc_call, grpc_serve, grpc_register_handler};
            self.datas.push(Var::new(String::from("grpc_register_service"), grpc_register_service as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("grpc_call"), grpc_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("grpc.serve"), grpc_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("grpc_register_handler"), grpc_register_handler as fn(String) -> Box<dyn Validator>, false));

            // Proxy / Reverse Proxy
            use crate::native::proxy::{proxy_forward, proxy_register, proxy_use, proxy_reverse_serve};
            self.datas.push(Var::new(String::from("proxy_forward"), proxy_forward as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("proxy.forward"), proxy_forward as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("proxy_register"), proxy_register as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("proxy.use"), proxy_use as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("proxy_reverse_serve"), proxy_reverse_serve as fn(String) -> Box<dyn Validator>, false));

            // RPC
            use crate::native::rpc::{rpc_register, rpc_call, rpc_serve, rpc_broadcast};
            self.datas.push(Var::new(String::from("rpc_register"), rpc_register as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc.register"), rpc_register as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc_call"), rpc_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc.call"), rpc_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc_serve"), rpc_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc.serve"), rpc_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc_broadcast"), rpc_broadcast as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("rpc.broadcast"), rpc_broadcast as fn(String) -> Box<dyn Validator>, false));

            // Microservices
            use crate::native::microservices::{ms_register, ms_discover, ms_call, ms_register_instance, ms_list};
            self.datas.push(Var::new(String::from("ms_register"), ms_register as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms.register"), ms_register as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms_discover"), ms_discover as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms.discover"), ms_discover as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms_call"), ms_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms.call"), ms_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms_register_instance"), ms_register_instance as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ms.list"), ms_list as fn(String) -> Box<dyn Validator>, false));

            // Serverless
            use crate::native::serverless::{sl_define, sl_invoke, sl_list, sl_invoke_count, sl_serve};
            self.datas.push(Var::new(String::from("sl_define"), sl_define as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl.define"), sl_define as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl_invoke"), sl_invoke as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl.invoke"), sl_invoke as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl_list"), sl_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl.invoke_count"), sl_invoke_count as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl_serve"), sl_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("sl.serve"), sl_serve as fn(String) -> Box<dyn Validator>, false));

            // Game Development (2D/3D rendering, SDL2, wgpu, sprites, tilemaps)
            use crate::native::game::{
                game_create_window, game_set_canvas, game_load_sprite, game_draw_sprite,
                game_rect, game_circle, game_line, game_text, game_clear, game_present,
                game_loop, game_get_keys, game_set_key, game_get_mouse,
                game_load_sound, game_play_sound, game_stop_sound,
                game_wgpu_init, game_wgpu_draw_triangle, game_wgpu_draw_rect, game_wgpu_present, game_wgpu_serve,
                game_3d_scene, game_3d_add_mesh, game_3d_set_camera, game_3d_render, game_3d_animate,
                game_sheet, game_tilemap, game_tile, game_map,
                game_unity_init, game_unity_call, game_unity_send_message, game_unity_register_external,
                game_unreal_init, game_unreal_call, game_unreal_register_function, game_unreal_send_event,
            };
            self.datas.push(Var::new(String::from("game_create_window"), game_create_window as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.set_canvas"), game_set_canvas as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_load_sprite"), game_load_sprite as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.draw_sprite"), game_draw_sprite as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_rect"), game_rect as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.circle"), game_circle as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.line"), game_line as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.text"), game_text as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_clear"), game_clear as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.present"), game_present as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_loop"), game_loop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.get_keys"), game_get_keys as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.set_key"), game_set_key as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.get_mouse"), game_get_mouse as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_load_sound"), game_load_sound as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.play_sound"), game_play_sound as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.stop_sound"), game_stop_sound as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_wgpu_init"), game_wgpu_init as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.wgpu_draw_triangle"), game_wgpu_draw_triangle as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.wgpu_draw_rect"), game_wgpu_draw_rect as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.wgpu_present"), game_wgpu_present as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.wgpu_serve"), game_wgpu_serve as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_3d_scene"), game_3d_scene as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_3d_add_mesh"), game_3d_add_mesh as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_3d_set_camera"), game_3d_set_camera as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_3d_render"), game_3d_render as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_3d_animate"), game_3d_animate as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_sheet"), game_sheet as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.tilemap"), game_tilemap as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.tile"), game_tile as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game_map"), game_map as fn(String) -> Box<dyn Validator>, false));
            // Unity Bridge
            self.datas.push(Var::new(String::from("game.unity_init"), game_unity_init as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unity_call"), game_unity_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unity_send_message"), game_unity_send_message as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unity_register_external"), game_unity_register_external as fn(String) -> Box<dyn Validator>, false));
            // Unreal Engine Bridge
            self.datas.push(Var::new(String::from("game.unreal_init"), game_unreal_init as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unreal_call"), game_unreal_call as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unreal_register_function"), game_unreal_register_function as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("game.unreal_send_event"), game_unreal_send_event as fn(String) -> Box<dyn Validator>, false));

            // Data Science
            use crate::native::data_science::{
                df_read_csv, df_head, df_shape, df_columns, df_filter, df_group_by, df_sort,
                stats_mean, stats_median, stats_stddev, stats_correlation, stats_min, stats_max, stats_summary,
                excel_read, excel_write, parquet_read, parquet_write,
                viz_histogram, viz_bar, viz_line, viz_scatter, viz_pie, viz_boxplot,
                df_select, df_rename, df_drop, df_merge,
            };
            // DataFrame
            self.datas.push(Var::new(String::from("df_read_csv"), df_read_csv as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.read_csv"), df_read_csv as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_head"), df_head as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.head"), df_head as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_shape"), df_shape as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.shape"), df_shape as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_columns"), df_columns as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.columns"), df_columns as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_filter"), df_filter as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.filter"), df_filter as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_group_by"), df_group_by as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.group_by"), df_group_by as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_sort"), df_sort as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.sort"), df_sort as fn(String) -> Box<dyn Validator>, false));
            // Statistics
            self.datas.push(Var::new(String::from("stats_mean"), stats_mean as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats.median"), stats_median as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats_stddev"), stats_stddev as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats.correlation"), stats_correlation as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats_min"), stats_min as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats.max"), stats_max as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("stats_summary"), stats_summary as fn(String) -> Box<dyn Validator>, false));
            // Excel
            self.datas.push(Var::new(String::from("excel_read"), excel_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("excel.read"), excel_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("excel_write"), excel_write as fn(String) -> Box<dyn Validator>, false));
            // Parquet
            self.datas.push(Var::new(String::from("parquet_read"), parquet_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("parquet.read"), parquet_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("parquet_write"), parquet_write as fn(String) -> Box<dyn Validator>, false));
            // Visualization
            self.datas.push(Var::new(String::from("viz_histogram"), viz_histogram as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz.histogram"), viz_histogram as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz_bar"), viz_bar as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz.bar"), viz_bar as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz_line"), viz_line as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz.line"), viz_line as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz_scatter"), viz_scatter as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz.scatter"), viz_scatter as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz_pie"), viz_pie as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("viz.boxplot"), viz_boxplot as fn(String) -> Box<dyn Validator>, false));
            // Data Transform
            self.datas.push(Var::new(String::from("df_select"), df_select as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.rename"), df_rename as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df_drop"), df_drop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("df.merge"), df_merge as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_run_in_shell"), shell_run_in_shell as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.run_in_shell"), shell_run_in_shell as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_env_get"), shell_env_get as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.env_get"), shell_env_get as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_env_set"), shell_env_set as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.env_set"), shell_env_set as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_env_remove"), shell_env_remove as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.env_remove"), shell_env_remove as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_env_list"), shell_env_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.env_list"), shell_env_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_cd"), shell_cd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.cd"), shell_cd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_pwd"), shell_pwd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.pwd"), shell_pwd as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_which"), shell_which as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.which"), shell_which as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_is_command"), shell_is_command as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.is_command"), shell_is_command as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_signal"), shell_signal as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.signal"), shell_signal as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_process_list"), shell_process_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.process_list"), shell_process_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_exit"), shell_exit as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.exit"), shell_exit as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_pty"), shell_pty as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.pty"), shell_pty as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_thread_spawn"), shell_thread_spawn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.thread_spawn"), shell_thread_spawn as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_thread_pool"), shell_thread_pool as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.thread_pool"), shell_thread_pool as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell_multi_exec"), shell_multi_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("shell.multi_exec"), shell_multi_exec as fn(String) -> Box<dyn Validator>, false));

            // System functions (bash, powershell, batch, daemons, services, drivers, kernel, bootloaders)
            use crate::native::system::*;
            // Bash
            self.datas.push(Var::new(String::from("bash_exec"), bash_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bash.exec"), bash_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bash_run_script"), bash_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bash.run_script"), bash_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bash_eval"), bash_eval as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bash.eval"), bash_eval as fn(String) -> Box<dyn Validator>, false));
            // PowerShell
            self.datas.push(Var::new(String::from("powershell_exec"), powershell_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("powershell.exec"), powershell_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("powershell_run_script"), powershell_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("powershell.run_script"), powershell_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("powershell_eval"), powershell_eval as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("powershell.eval"), powershell_eval as fn(String) -> Box<dyn Validator>, false));
            // Batch
            self.datas.push(Var::new(String::from("batch_exec"), batch_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("batch.exec"), batch_exec as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("batch_run_script"), batch_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("batch.run_script"), batch_run_script as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("batch_eval"), batch_eval as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("batch.eval"), batch_eval as fn(String) -> Box<dyn Validator>, false));
            // Daemons
            self.datas.push(Var::new(String::from("daemon_start"), daemon_start as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon.start"), daemon_start as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon_stop"), daemon_stop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon.stop"), daemon_stop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon_status"), daemon_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon.status"), daemon_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon_restart"), daemon_restart as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("daemon.restart"), daemon_restart as fn(String) -> Box<dyn Validator>, false));
            // Services
            self.datas.push(Var::new(String::from("service_create"), service_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.create"), service_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_start"), service_start as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.start"), service_start as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_stop"), service_stop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.stop"), service_stop as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_restart"), service_restart as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.restart"), service_restart as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_status"), service_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.status"), service_status as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_enable"), service_enable as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.enable"), service_enable as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_disable"), service_disable as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.disable"), service_disable as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service_list"), service_list as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("service.list"), service_list as fn(String) -> Box<dyn Validator>, false));
            // Drivers
            self.datas.push(Var::new(String::from("driver_load"), driver_load as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver.load"), driver_load as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver_unload"), driver_unload as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver.unload"), driver_unload as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver_list_devices"), driver_list_devices as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver.list_devices"), driver_list_devices as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver_sysfs_read"), driver_sysfs_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver.sysfs_read"), driver_sysfs_read as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver_sysfs_write"), driver_sysfs_write as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("driver.sysfs_write"), driver_sysfs_write as fn(String) -> Box<dyn Validator>, false));
            // Kernel
            self.datas.push(Var::new(String::from("kernel_version"), kernel_version as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.version"), kernel_version as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_list_modules"), kernel_list_modules as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.list_modules"), kernel_list_modules as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_load_module"), kernel_load_module as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.load_module"), kernel_load_module as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_unload_module"), kernel_unload_module as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.unload_module"), kernel_unload_module as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_module_info"), kernel_module_info as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.module_info"), kernel_module_info as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_dmesg"), kernel_dmesg as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.dmesg"), kernel_dmesg as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_sysctl_get"), kernel_sysctl_get as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.sysctl_get"), kernel_sysctl_get as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel_sysctl_set"), kernel_sysctl_set as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("kernel.sysctl_set"), kernel_sysctl_set as fn(String) -> Box<dyn Validator>, false));
            // Bootloaders
            self.datas.push(Var::new(String::from("bootloader_list_entries"), bootloader_list_entries as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader.list_entries"), bootloader_list_entries as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader_set_default"), bootloader_set_default as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader.set_default"), bootloader_set_default as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader_get_default"), bootloader_get_default as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader.get_default"), bootloader_get_default as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader_install"), bootloader_install as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader.install"), bootloader_install as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader_update"), bootloader_update as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("bootloader.update"), bootloader_update as fn(String) -> Box<dyn Validator>, false));

            // AI/ML
            use crate::native::ml::{
                ml_create_session, ml_list_sessions, ml_infer, ml_load_model, ml_list_models,
                ml_onnx_session, ml_onnx_run, ml_candle_train,
                llm_load_gguf, llm_generate, llm_chat,
                embed_create, embed_encode,
                ml_tensor_create, ml_tensor_run, ml_tf_model, ml_torch_model,
                ml_backend_info, ml_help,
            };
            // Core ML
            self.datas.push(Var::new(String::from("ml.create_session"), ml_create_session as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.create"), ml_create_session as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_list_sessions"), ml_list_sessions as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.list_sessions"), ml_list_sessions as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_infer"), ml_infer as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.infer"), ml_infer as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_load_model"), ml_load_model as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.load_model"), ml_load_model as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_list_models"), ml_list_models as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.list_models"), ml_list_models as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_backend_info"), ml_backend_info as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.backend_info"), ml_backend_info as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_help"), ml_help as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.help"), ml_help as fn(String) -> Box<dyn Validator>, false));
            // ONNX
            self.datas.push(Var::new(String::from("ml_onnx_session"), ml_onnx_session as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.onnx_session"), ml_onnx_session as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_onnx_run"), ml_onnx_run as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.onnx_run"), ml_onnx_run as fn(String) -> Box<dyn Validator>, false));
            // Candle
            self.datas.push(Var::new(String::from("ml_candle_train"), ml_candle_train as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.candle_train"), ml_candle_train as fn(String) -> Box<dyn Validator>, false));
            // Tensors
            self.datas.push(Var::new(String::from("ml_tensor_create"), ml_tensor_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.tensor"), ml_tensor_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_tensor_run"), ml_tensor_run as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.tensor.run"), ml_tensor_run as fn(String) -> Box<dyn Validator>, false));
            // TF / Torch
            self.datas.push(Var::new(String::from("ml_tf_model"), ml_tf_model as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.tf_model"), ml_tf_model as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml_torch_model"), ml_torch_model as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("ml.torch_model"), ml_torch_model as fn(String) -> Box<dyn Validator>, false));
            // LLMs
            self.datas.push(Var::new(String::from("llm_load_gguf"), llm_load_gguf as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("llm.load_gguf"), llm_load_gguf as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("llm_generate"), llm_generate as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("llm.generate"), llm_generate as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("llm_chat"), llm_chat as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("llm.chat"), llm_chat as fn(String) -> Box<dyn Validator>, false));
            // Embeddings
            self.datas.push(Var::new(String::from("embed_create"), embed_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("embed.create"), embed_create as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("embed_encode"), embed_encode as fn(String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("embed.encode"), embed_encode as fn(String) -> Box<dyn Validator>, false));

            // Android stdlib
            #[cfg(feature = "android")]
            {
                crate::stdlib::android::init_android_stdlib();
            }

            match &self.action {
                Some(act) => match act {
                    Act::Run => self.run_code(file),
                    Act::Cli => {}
                    Act::Comp => {}
                },
                None => {
                    eprintln!("RuntimeError: nenhuma ação foi definida antes de chamar run().");
                }
            };
        }

        fn run_code(&mut self, path: String) {
            let file_to_run = read_file(path);
            let codes: Vec<&str> = file_to_run.trim().split('\n').collect();
            get_lexer(codes);
        }

        // ── Variable manager ─────────────────────────────────────────────────

        pub fn get_vars(&self) -> &Vec<Var> {
            &self.datas
        }


        // Schema manager
        pub fn register_var_data(&mut self, name: String, value: ValueData) {
            self.datas.push(Var::new(name, value, true));
        }

        pub fn create_shared_var(&mut self, name: String, target_var_name: String) -> Result<(), String> {
            let var = self.get_var_per_name(target_var_name.clone())?;
            let shared_val = ValueData::new_shared(var.get_value());
            self.register_var_data(name, shared_val);
            Ok(())
        }

        pub fn register_function(&mut self, name: String, func_val: ValueData) {
            self.datas.push(Var::new(name.clone(), func_val, true));
        }

        pub fn register_schema(&mut self, lexers: Vec<Lexer>) -> Result<(), String> {
            if lexers.is_empty() {
                return Err(String::from("Esperado nome de Schema após 'Schema'."));
            }
            let name = match lexers.get(1) {
                Some(Lexer { token: Tokens::Reference | Tokens::Identifier, literal, .. }) => literal.clone(),
                _ => return Err(String::from("Esperado nome de Schema após 'Schema'.")),
            };
            let mut schema_def = SchemaDef::new(name.clone());
            let mut i = 2;
            if i < lexers.len() && lexers[i].token == Tokens::Extends {
                i += 1;
                if i < lexers.len() {
                    if matches!(lexers[i].token, Tokens::Reference | Tokens::Identifier) {
                        schema_def.parent = Some(lexers[i].literal.clone());
                        i += 1;
                    }
                }
            }
            while i < lexers.len() && lexers[i].token != Tokens::LeftBrace { i += 1; }
            if i >= lexers.len() { return Err(String::from("Esperado '{' após nome do Schema.")); }
            i += 1;
            let body: Vec<Lexer> = lexers[i..].iter().filter(|l| l.token != Tokens::RightBrace).cloned().collect();
            let mut j = 0;
            while j < body.len() {
                let token = &body[j].token;
                if token == &Tokens::Identifier && body[j].literal == "init" {
                    let mut constructor_tokens = vec![];
                    j += 1;
                    while j < body.len() {
                        let t = &body[j].token;
                        if t == &Tokens::LeftBrace {
                            constructor_tokens.push(body[j].clone());
                            j += 1;
                            let mut depth = 1;
                            while j < body.len() && depth > 0 {
                                if body[j].token == Tokens::LeftBrace { depth += 1; }
                                if body[j].token == Tokens::RightBrace { depth -= 1; }
                                constructor_tokens.push(body[j].clone());
                                j += 1;
                            }
                            break;
                        }
                        j += 1;
                    }
                    schema_def.constructor_body = Some(constructor_tokens);
                    continue;
                }
                if token == &Tokens::Identifier && body[j].literal == "static" {
                    j += 1;
                    if j < body.len() && body[j].token == Tokens::Identifier {
                        let method_name = body[j].literal.clone();
                        j += 1;
                        let mut params = vec![];
                        if j < body.len() && body[j].token == Tokens::LeftParenthesis {
                            j += 1;
                            while j < body.len() && body[j].token != Tokens::RightParenthesis {
                                if body[j].token == Tokens::Identifier { params.push(body[j].literal.clone()); }
                                j += 1;
                            }
                            if j < body.len() && body[j].token == Tokens::RightParenthesis { j += 1; }
                        }
                        let mut method_body = vec![];
                        if j < body.len() && body[j].token == Tokens::LeftBrace {
                            j += 1;
                            let mut depth = 1;
                            while j < body.len() && depth > 0 {
                                if body[j].token == Tokens::LeftBrace { depth += 1; }
                                if body[j].token == Tokens::RightBrace { depth -= 1; }
                                method_body.push(body[j].clone());
                                j += 1;
                            }
                        }
                        schema_def.add_static_method(method_name, params, method_body);
                        continue;
                    }
                }
                if token == &Tokens::Identifier && j + 1 < body.len() && body[j+1].token == Tokens::LeftParenthesis {
                    let method_name = body[j].literal.clone();
                    j += 1;
                    let mut params = vec![];
                    while j < body.len() && body[j].token != Tokens::RightParenthesis {
                        if body[j].token == Tokens::Identifier { params.push(body[j].literal.clone()); }
                        j += 1;
                    }
                    if j < body.len() && body[j].token == Tokens::RightParenthesis { j += 1; }
                    let mut method_body = vec![];
                    if j < body.len() && body[j].token == Tokens::LeftBrace {
                        j += 1;
                        let mut depth = 1;
                        while j < body.len() && depth > 0 {
                            if body[j].token == Tokens::LeftBrace { depth += 1; }
                            if body[j].token == Tokens::RightBrace { depth -= 1; }
                            method_body.push(body[j].clone());
                            j += 1;
                        }
                    }
                    schema_def.add_method(method_name, params, method_body);
                    continue;
                }
                if token == &Tokens::Identifier {
                    let mut field_name = body[j].literal.clone();
                    if let Some(colon_idx) = field_name.find(':') {
                        field_name = field_name[..colon_idx].trim().to_string();
                    }
                    j += 1;
                    let mut default_tokens = vec![];
                    if j < body.len() && body[j].token == Tokens::Colon {
                        j += 1;
                        while j < body.len() && body[j].token != Tokens::Comma {
                            default_tokens.push(body[j].clone());
                            j += 1;
                        }
                    } else if j < body.len() && body[j].token == Tokens::Comma {
                        j += 1;
                    }
                    schema_def.add_field(field_name, default_tokens);
                    continue;
                }
                j += 1;
            }
            if let Some(idx) = self.schemas.iter().position(|s| s.name == name) {
                self.schemas[idx] = schema_def;
            } else {
                self.schemas.push(schema_def);
            }
            Ok(())
        }

        pub fn drop_var(&mut self, name: &str) {
            self.datas.retain(|v| v.get_name() != name);
            crate::runtime::memory::with_heap(|h| h.forget_name(name));
        }

        pub fn get_schema(&self, name: &str) -> Option<&SchemaDef> {
            self.schemas.iter().find(|s| s.name == name)
        }

        pub fn register_type_alias(&mut self, name: String, target_type: Type) {
            self.type_aliases.insert(name, target_type);
        }

        pub fn assert_condition(&self, condition: bool, message: String) -> Result<(), String> {
            if !condition {
                return Err(format!("Assertion failed: {}", message));
            }
            Ok(())
        }

        pub fn set_strict_mode(&mut self, enabled: bool) {
            self.strict_mode = enabled;
        }

        pub fn create_instance(&self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            if lexers.len() < 2 {
                eprintln!("SyntaxError: esperado nome do Schema após 'new'.");
                return Box::new(ValueData::Object(Object::from_map(linked_hash_map::LinkedHashMap::new())));
            }
            let schema_name = match &lexers[1].token {
                Tokens::Identifier | Tokens::Reference => lexers[1].literal.clone(),
                _ => {
                    eprintln!("SyntaxError: esperado nome de Schema após 'new'.");
                    return Box::new(ValueData::Object(Object::from_map(linked_hash_map::LinkedHashMap::new())));
                }
            };
            let schema_def = match self.get_schema(&schema_name) {
                Some(s) => s,
                None => {
                    eprintln!("TypeError na linha {}: Schema '{}' não encontrado.", lexers[1].line, schema_name);
                    return Box::new(ValueData::Object(Object::from_map(linked_hash_map::LinkedHashMap::new())));
                }
            };
            let mut obj_map = linked_hash_map::LinkedHashMap::new();
            let fields = schema_def.get_all_fields(&self.schemas);
            for (field_name, default_expr) in fields {
                if !default_expr.is_empty() {
                    let value = process_value(default_expr);
                    obj_map.insert(field_name.clone(), value);
                } else {
                    obj_map.insert(field_name.clone(), ValueData::String(String::from("None")));
                }
            }
            obj_map.insert(String::from("__instance__"), ValueData::String(format!("__instance__:{}", schema_name)));
            // Parse constructor arguments
            let mut args = vec![];
            let mut paren_depth = 0;
            let mut arg_tokens = vec![];
            for lex in &lexers[2..] {
                if lex.token == Tokens::LeftParenthesis {
                    paren_depth += 1;
                    if paren_depth > 1 { arg_tokens.push(lex.clone()); }
                } else if lex.token == Tokens::RightParenthesis {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        if !arg_tokens.is_empty() {
                            args.push(process_value(arg_tokens.clone()));
                            arg_tokens.clear();
                        }
                        break;
                    } else {
                        arg_tokens.push(lex.clone());
                    }
                } else if lex.token == Tokens::Comma && paren_depth == 1 {
                    if !arg_tokens.is_empty() {
                        args.push(process_value(arg_tokens.clone()));
                        arg_tokens.clear();
                    }
                } else if paren_depth >= 1 {
                    arg_tokens.push(lex.clone());
                }
            }

            // Recursive constructor caller
            fn call_constructors(
                aly: &mut Aly,
                schema_name: &str,
                schema_defs: &[SchemaDef],
                obj: &mut Object,
                args: &[ValueData],
            ) {
                if let Some(schema_def) = schema_defs.iter().find(|s| s.name == schema_name) {
                    if let Some(ref parent_name) = schema_def.parent {
                        call_constructors(aly, parent_name, schema_defs, obj, args);
                    }
                    if let Some(ref init_body) = schema_def.constructor_body {
                        let mut init_tokens = init_body.clone();
                        let mut dummy_val: Box<dyn Validator> = Box::new(String::new());
                        
                        // Bind arguments to local scope for this constructor
                        for (i, arg) in args.iter().enumerate() {
                            let var_name = format!("arg{}", i);
                            aly.register_var_data(var_name, arg.clone());
                        }

                        exec(&mut init_tokens, &mut dummy_val);
                    }
                }
            }

            let mut obj = Object::from_map(obj_map);
            
            // Register 'this' and 'self' alias for the instance
            let obj_val = ValueData::Object(obj.clone());
            get_runtime().register_var_data(String::from("this"), obj_val.clone());
            get_runtime().register_var_data(String::from("self"), obj_val);

            call_constructors(get_runtime(), &schema_name, &self.schemas, &mut obj, &args);
            
            Box::new(ValueData::Object(obj))
        }

        pub fn create_variable(&mut self, lexers: Vec<Lexer>) {
            if lexers.is_empty() {
                eprintln!("SyntaxError: instrução de variável vazia.");
                return;
            }

            if lexers[0].token.id() != "def_let" {
                // Reassignment: `name = value` or compound: `name += value`
                let name = &lexers[0];
                let op_token = match lexers.get(1) {
                    Some(l) => &l.token,
                    None => {
                        eprintln!("SyntaxError: esperado '=' após o nome da variável na linha {}.", name.line);
                        return;
                    }
                };

                // Check for compound assignment operators
                let compound_op = match op_token {
                    Tokens::PlusEqual => Some("+"),
                    Tokens::MinusEqual => Some("-"),
                    Tokens::TimesEqual => Some("*"),
                    Tokens::DivideEqual => Some("/"),
                    Tokens::ModulusEqual => Some("%"),
                    Tokens::Identifier => None, // regular assignment
                    _ => {
                        eprintln!(
                            "SyntaxError: esperado '=' ou operador de atribuição composto após o nome da variável na linha {}.",
                            name.line
                        );
                        return;
                    }
                };

                let value_lexers = if compound_op.is_some() {
                    // For compound: get the value after the operator
                    if lexers.len() < 3 {
                        eprintln!("SyntaxError: esperado valor após o operador na linha {}.", name.line);
                        return;
                    }
                    // Get current value and combine with new value
                    match self.get_var(name.clone()) {
                        Ok(v) => {
                            let current_val = v.get_value().to_string(false);
                            let new_val = process_value(lexers[2..].to_vec()).to_string(false);
                            // Build expression: current op new_value
                            let op = compound_op.unwrap();
                            let combined = format!("{} {} {}", current_val, op, new_val);
                            vec![Lexer::new(Tokens::Value, combined, name.line)]
                        }
                        Err(err) => {
                            eprintln!("ReferenceError na linha {}: {}", name.line, err);
                            return;
                        }
                    }
                } else {
                    // Regular assignment: get the value after '='
                    lexers[2..].to_vec()
                };

                match self.get_var(name.clone()) {
                    Ok(v) => {
                        if v.is_borrowed() {
                            eprintln!("BorrowError: variável '{}' está emprestada.", name.literal);
                            return;
                        }
                        let raw = process_value(value_lexers);
                        let ty = v.get_type().clone();
                        let final_value = match ty {
                            Type::Fixed(_) | Type::Pointer(_) => match coerce(raw, &ty) {
                                Ok(x) => x,
                                Err(e) => {
                                    eprintln!("TypeError: {}", e);
                                    return;
                                }
                            },
                            _ => raw,
                        };
                        if let Err(err) = v.change_value(final_value) {
                            eprintln!("TypeError na linha {}: {}", name.line, err);
                        }
                    }
                    Err(err) => {
                        eprintln!("ReferenceError na linha {}: {}", name.line, err);
                    }
                }

                return;
            }

            // Declaration: `let name = value`
            let name = match lexers.get(1) {
                Some(l) => l.clone(),
                None => {
                    eprintln!("SyntaxError: esperado nome após 'let'.");
                    return;
                }
            };

            if lexers.len() == 2 {
                // `let x` — initialise to None
                let var = Var::new(name.literal.to_string(), ValueData::String("None".to_owned()).to_string(false), true);
                self.datas.push(var);
                return;
            }

            let identifier = match lexers.get(2) {
                Some(l) => l,
                None => {
                    eprintln!("SyntaxError: esperado '=' após o nome da variável na linha {}.", name.line);
                    return;
                }
            };

            if identifier.token == Tokens::Colon {
                // Declaração tipada: `let name : i8 = value`
                if lexers.len() < 4 {
                    eprintln!(
                        "SyntaxError: esperado o tipo após ':' na linha {}.",
                        name.line
                    );
                    return;
                }
                let ty_name = lexers[3].literal.clone();
                let ty = match parse_type_annotation(&ty_name) {
                    Some(t) => t,
                    None => {
                        eprintln!(
                            "TypeError: tipo desconhecido '{}' na linha {}.",
                            ty_name, name.line
                        );
                        return;
                    }
                };

                if lexers.len() == 4 {
                    // `let x : i8` — sem valor inicial
                    match Var::new_typed(
                        name.literal.to_string(),
                        ValueData::String("None".to_owned()),
                        true,
                        ty,
                    ) {
                        Ok(var) => self.datas.push(var),
                        Err(e) => eprintln!("TypeError: {}", e),
                    }
                    return;
                }

                if lexers.get(4).map(|l| l.token.clone()) != Some(Tokens::Identifier) {
                    eprintln!(
                        "SyntaxError: esperado '=' após o tipo na linha {}.",
                        name.line
                    );
                    return;
                }

                let value = process_value(lexers[5..].to_vec());
                match Var::new_typed(name.literal.to_string(), value, true, ty) {
                    Ok(var) => self.datas.push(var),
                    Err(e) => eprintln!("TypeError: {}", e),
                }
                return;
            }

            if identifier.token != Tokens::Identifier {
                eprintln!(
                    "SyntaxError: esperado '=' após o nome da variável na linha {}.",
                    name.line
                );
                return;
            }

            let value = process_value(lexers[3..].to_vec());
            let var = Var::new(name.literal.to_string(), value, true);
            self.datas.push(var);
        }

        pub fn create_constant(&mut self, lexers: Vec<Lexer>) {
            if lexers.len() < 2 {
                eprintln!("SyntaxError: esperado nome após 'const'.");
                return;
            }

            let name = lexers[1].clone();

            if lexers.len() == 2 {
                // `const x` without a value — silently skip (or warn)
                return;
            }

            let identifier = match lexers.get(2) {
                Some(l) => l,
                None => {
                    eprintln!("SyntaxError: esperado '=' após o nome da constante na linha {}.", name.line);
                    return;
                }
            };

            if identifier.token != Tokens::Identifier {
                eprintln!(
                    "SyntaxError: esperado '=' após o nome da constante na linha {}.",
                    name.line
                );
                return;
            }

            let value = process_value(lexers[3..].to_vec());
            let constant = Var::new(name.literal.to_string(), value, false);
            self.datas.push(constant);
        }

        pub fn get_var(&mut self, name: Lexer) -> Result<&mut Var, String> {
            let var = self
                .datas
                .iter_mut()
                .find(|var| var.compare_var(name.literal.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!(
                    "Variável '{}' não existe.\n\nlinha: {}",
                    name.literal, name.line,
                )),
            }
        }

        pub fn get_var_per_name(&mut self, name: String) -> Result<&mut Var, String> {
            let var = self
                .datas
                .iter_mut()
                .find(|var| var.compare_var(name.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!("Variável '{}' não existe.", name)),
            }
        }

        pub fn register_plugin_function(
            &mut self,
            var_name: String,
            namespace: String,
            func_name: String,
        ) {
            self.datas.push(Var::new(
                var_name,
                ValueData::PluginFunction {
                    namespace,
                    func_name,
                },
                false,
            ));
        }

        pub fn get_var_prop(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            if lexers.is_empty() {
                eprintln!("InternalError: get_var_prop chamado com lista de lexers vazia.");
                return Box::new("None".to_owned());
            }

            match self.get_var(lexers[0].clone()) {
                Ok(var) => var.get_prop(lexers[1..].to_vec()),
                Err(err) => {
                    eprintln!("ReferenceError: {}", err);
                    Box::new("None".to_owned())
                }
            }
        }

        // ── Function execution ───────────────────────────────────────────────

        pub fn function_run(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            if lexers.is_empty() {
                eprintln!("SyntaxError: chamada de função vazia.");
                return Box::new("None".to_owned());
            }

            // Handle `new SchemaName(...)` constructor calls
            if lexers[0].token == Tokens::New {
                return self.create_instance(lexers);
            }

            // Find opening parenthesis position
            let paren_idx = match lexers.iter().position(|l| l.token == Tokens::LeftParenthesis) {
                Some(i) => i,
                None => {
                    eprintln!("SyntaxError: chamada de função inválida (sem parênteses).");
                    return Box::new("None".to_owned());
                }
            };

            // Full function name or method call (e.g. `obj.method()`)
            let mut name_parts = Vec::new();
            for lex in &lexers[..paren_idx] {
                if lex.token != Tokens::Dot {
                    name_parts.push(lex.literal.clone());
                }
            }
            
            // Check if it's a method call on an object/instance
            if name_parts.len() >= 2 {
                let obj_name = name_parts[0].clone();
                let method_name = name_parts[1].clone();
                
                // Get object and check instance type
                if let Ok(var) = self.get_var_per_name(obj_name) {
                    if let ValueData::Object(obj) = var.get_value() {
                        // Check if object is a Schema Instance
                        if let Some(instance_tag) = obj.get_item(String::from("__instance__")).to_string(false).strip_prefix("__instance__:") {
                            if let Some(schema_def) = self.schemas.iter().find(|s| s.name == instance_tag) {
                                // Dynamic dispatch: lookup method in schema hierarchy
                                if let Some((_params, _body)) = schema_def.get_method(&method_name, &self.schemas) {
                                    eprintln!("DEBUG: Found method '{}' for Schema '{}'", method_name, instance_tag);
                                    // Here we would bind 'this' to the object instance
                                    // and execute the body tokens.
                                    return Box::new(ValueData::String(format!("<method {} called>", method_name)));
                                }
                            }
                        }
                    }
                }
            }

            let mut params: Vec<Lexer> = vec![];
            let mut fun_body: Vec<Lexer> = vec![];
            let mut another_fun = 0;

            // Parse arguments between parenteses
            let args_range = if paren_idx + 1 < lexers.len() - 1 {
                &lexers[paren_idx + 1..lexers.len() - 1]
            } else {
                &[]
            };

            for lex in args_range {
                if lex.literal == "," {
                    continue;
                } else if is_opened(lex.token.clone()) {
                    another_fun += 1;

                    if another_fun == 1 {
                        let ind = params.len().saturating_sub(1);
                        if !params.is_empty() {
                            fun_body.push(lexers[paren_idx + 1 + ind].clone());
                        }
                    }

                    fun_body.push(lex.clone());
                    params.pop();
                } else if is_close(lex.token.clone()) {
                    another_fun -= 1;
                    fun_body.push(lex.clone());

                    if another_fun == 0 {
                        let res = process_value(fun_body.clone());
                        let lexer_res = Lexer::new(Tokens::Value, res.to_string(true), lex.line);
                        params.push(lexer_res);
                        fun_body.clear();
                    }
                } else if another_fun > 0 {
                    fun_body.push(lex.clone());
                } else {
                    params.push(lex.clone());
                }
            }

            let name_str = name_parts.join(".");
            let name_lex = Lexer::new(Tokens::None, name_str.clone(), lexers[0].line);

            match self.get_var(name_lex.clone()) {
                Ok(ok) => {
                    match ok.get_type() {
                        Type::NativeFunction | Type::Function | Type::PluginFunction => {
                            match ok.get_value() {
                                ValueData::NativeFunction(fun) => {
                                    let param = remove_quoted_str(
                                        process_value(params).to_string(false),
                                    );
                                    fun(param)
                                }
                                ValueData::NativeData(fun) => {
                                    let mut args = Vec::with_capacity(params.len());
                                    for p in &params {
                                        args.push(process_value(vec![p.clone()]).valid().1);
                                    }
                                    Box::new(fun(&args))
                                }
                                ValueData::PluginFunction { namespace, func_name } => {
                                    let param = params.iter()
                                        .map(|p| remove_quoted_str(
                                            process_value(vec![p.clone()]).to_string(false)
                                        ))
                                        .collect::<Vec<_>>()
                                        .join(",");
                                    let result = call_plugin(&namespace, &func_name, &param);
                                    Box::new(result)
                                }
                                _ => Box::new("None".to_owned()),
                            }
                        }
                        _ => {
                            eprintln!(
                                "TypeError na linha {}: '{}' não é uma função.",
                                name_lex.line, name_lex.literal
                            );
                            Box::new("None".to_owned())
                        }
                    }
                }
                Err(err) => {
                    eprintln!("ReferenceError: {}", err);
                    Box::new("None".to_owned())
                }
            }
        }
    }
}

pub use aly::*;