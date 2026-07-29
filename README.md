# Aly Programming Language

Aly is a dynamically-typed programming language that balances speed (AOT compilation and JIT VM execution) with developer productivity. It features native GUI components, FFI extensibility, and compiler backends targeting multiple environments.

---

## Programming Guide & Documentation

Learn Aly step-by-step through our detailed guides:

### Core Language
* [Variables and Mutability](docs/variables.md): `let` vs `const`, dynamic type inference, and block-based lexical scoping.
* [Data Types](docs/data_types.md): Primitives (int, float, bool, char, string) and collections (list, object).
* [Operators](docs/operators.md): Arithmetic, comparison (eq, gt, lt, etc.), logical, and assignment operators.
* [Strings](docs/strings.md): String literals, interpolation ($var, &expr), and operations.
* [Functions](docs/functions.md): Named and anonymous functions, closures, default params, variadic args.
* [Dynamic Mutability Freezing](docs/constant_and_tomb.md): The `tomb()` freeze system.
* [Semicolons, Continuation & Comments](docs/syntax_details.md): Semicolon rules, line continuation, and comment formatting.
* [Modules](docs/modules.md): Import/export system for organizing code across files.

### Control Flow
* [Conditionals](docs/control_flow/conditional.md): if, elif, else and logical operators.
* [Loops](docs/control_flow/loop.md): Condition loops, for-style loops, break and continue.
* [Pattern Matching](docs/control_flow/match.md): Match complex patterns with wildcards.

### Object-Oriented Programming (OOP)
* [Objects and Methods](docs/poo/objects.md): Dynamic key-value structures with member functions.
* [Schemas (Class Blueprints)](docs/poo/Schema.md): Structured classes with constructors and field overrides.
* [Advanced Schemas](docs/schemas_advanced.md): Inheritance, static methods, generics, custom constructors.
* [Traits](docs/traits.md): Interface definitions and schema trait verification.

### Advanced Language Features
* [Concurrency](docs/concurrency.md): AtomicVar and thread spawning.
* [Reflection](docs/reflection.md): Runtime type inspection, listing variables and schemas.
* [Metaprogramming](docs/metaprogramming.md): Macros, lazy evaluation, compile-time constant folding.
* [JOT Data Format](docs/jot.md): JSON-like structured data format with query (pick).
* [Collections](docs/collections.md): Tuples, Sets, Stacks, Queues, Linked Lists, Enumerations.

### Standard Library
* [Built-in Core Libraries](docs/native_libraries.md): Overview of standard utilities.
* [Filesystem](docs/native_libraries/filesystem.md): File read/write, copy, move, directory operations.
* [System & Environment Utilities](docs/native_libraries/system.md): dotenv, uuid, console styling.
* [OS & Shell](docs/native_libraries/os_shell.md): OS info, environment, command execution, process management.
* [Math, Crypto & Regex](docs/native_libraries/math_crypto_regex.md): Trigonometry, hashes (SHA256, AES), regex engine.
* [Data Formats & Codecs](docs/native_libraries/data_formats.md): Base64/Hex, CSV, GZIP, Tar, PDF generation.
* [JSON](docs/native_libraries/json.md): JSON parsing and serialization.
* [Network & HTTP](docs/native_libraries/net.md): TCP/UDP sockets, HTTP client/server, WebSocket, MQTT, TLS.
* [Database](docs/native_libraries/database.md): SQLite, PostgreSQL, MySQL, Redis.
* [Security](docs/native_libraries/security.md): JWT, OAuth2, OIDC, TLS, hashing, signatures, ACME, sanitization.
* [Artificial Intelligence & ML](docs/ai_ml.md): Neural networks, GGUF LLMs, vector embeddings.
* [Data Science](docs/native_libraries/data_science.md): Statistics, linear algebra, data frames.

### Platform & Extensibility
* [Native Bindings & Plugins](docs/native.md): Rust native modules and dynamic library plugins.
* [Language Bindings](docs/bindings.md): Java/JNI, .NET, Node.js interop.
* [C++ ABI](docs/cpp_abi.md): C++ function calls, name mangling, struct layout.
* [Compiler Backends](docs/compiler_backends.md): LLVM, C++, Go, Rust, JS, Python, JVM, Kotlin transpilation.
* [Compiler Optimization](docs/compiler_optimization.md): Incremental, parallel compilation, JIT optimizations.

### GUI & Graphics
* [Graphical User Interfaces](docs/graphics/geral.md): Event-driven GUI with multiple backends.
  * [FLTK backend](docs/graphics/fltk.md)
  * [GTK4 backend](docs/graphics/gtk.md)
  * [Cocoa backend](docs/graphics/cocoa.md)
* [Graphics Rendering](docs/render.md): OpenGL, Vulkan, DirectX 3D rendering.

### Tools & Platform
* [Developer Tools](docs/tools/index.md): Formatter, linter, LSP, debugger, test runner, hot reload.
* [APG Package Manager](docs/apg.md): Package manager CLI for dependencies and publishing.
* [Android Support](docs/android.md): Build and package Aly apps for Android.
* [IoT & Embedded](docs/native_libraries/iot.md): ESP32, Arduino, Raspberry Pi, GPIO, I2C, SPI, UART.
* [Web Platform](docs/native_libraries/web.md): DOM, CSS, Web Components, WASM backend.
* [Game Development](docs/native_libraries/game.md): 2D game window, sprites, input, audio.
* [System Utilities](docs/system_utilities.md): Notifications, system tray, clipboard.
* [RPC & Microservices](docs/rpc_microservices.md): HTTP RPC, GraphQL, gRPC, proxy, serverless.
* [REPL & Scheduler](docs/repl.md): Interactive prompt and cooperative task scheduler.

---

## Quick Start

### 1. Execute script via VM (JIT interpreter)
```bash
cargo run --bin aly -- run path/to/script.aly
```

### 2. Compile directly to Machine Code (direct AOT)
Use the `alyc` compiler tool to compile Aly directly into Linux ELF, Windows PE .exe, or flat binaries .bin:
```bash
./target/release/alyc -o myapp.exe myapp.aly
./target/release/alyc -o myapp myapp.aly
```

### 3. Transpile to other languages
```bash
./target/release/alyc -o myapp.go myapp.aly
./target/release/alyc -o myapp.rs myapp.aly
./target/release/alyc -o myapp.sh myapp.aly
```