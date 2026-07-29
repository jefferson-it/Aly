# FALTA.md — Auditoria do código vs claims no `todo.md`

> Última auditoria: 28/07/2026 — Correções aplicadas.

---

## ✅ Correções Aplicadas

### Rodada 1 — AI/ML, Unity, Unreal, WASM

| Ação | Detalhes |
|------|----------|
| `todo.md` — 9 items desmarcados | TensorFlow, PyTorch, ONNX, CUDA, OpenCL, LLMs, Embeddings, Unity Bindings, Unreal Bindings |
| `Cargo.toml` — `ai-onnx` corrigido | `ai-onnx = []` → `ai-onnx = ["ort"]` + crate `ort` adicionado |
| `Cargo.toml` — `ai-embeddings` criado | Feature referenciada no código mas não existia |
| `src/native/mod.rs` — WASM ativado | `pub mod wasm_backend;` adicionado (544 linhas de dead code ativadas) |
| `todo.md` — caminho WASM corrigido | `src/compiler/wasm_backend.rs` → `src/native/wasm_backend.rs` |

### Rodada 2 — Web, Jogos, Dados, Arquivos, Ferramentas, Interop

| Ação | Detalhes |
|------|----------|
| `todo.md` — SSR desmarcado | DOM backend só gera HTML estático, sem SSR |
| `todo.md` — SSG desmarcado | Não implementado |
| `todo.md` — SPA desmarcado | Não implementado |
| `todo.md` — PWA desmarcado | Sem service worker ou manifest |
| `todo.md` — Web Components desmarcado | Sem custom elements ou shadow DOM |
| `todo.md` — OpenGL/Vulkan/DirectX desmarcado | Arquivos em `src/render/` sem `mod.rs` (dead code) + `ash` não está no Cargo.toml |
| `todo.md` — XML desmarcado (2x) | Nenhum parser XML, só strings de AndroidManifest |
| `todo.md` — YAML desmarcado | Nenhum parser YAML |
| `todo.md` — INI desmarcado | Nenhum parser INI |
| `todo.md` — Profiler desmarcado | Só PGO em `src/profile/guided.rs`, sem profiler real |
| `todo.md` — Benchmark corrigido | Removido C# e Node.js (só `.c`, `.cpp`, `.go`, `.rs` existem) |
| `todo.md` — .NET desmarcado | `src/bindings/dotnet.rs` é gerador de código C#, não integração runtime |
| `todo.md` — ONNX nota atualizada | `ort` agora está no Cargo.toml, mas `ml_onnx_run()` ainda é stub |

---

## ⚠️ Items que permanecem como stubs (código não corrigido)

Estes items foram **desmarcados no `todo.md`** mas o código continua sendo stub.
Para implementá-los de verdade, seria necessário:

| Item | O que falta |
|------|------------|
| TensorFlow | `ai-tensorflow` usa `tch` (PyTorch bindings), não TensorFlow. Nenhuma inferência real |
| PyTorch | Crate `tch` listada mas nunca importada (`use tch` não existe). Nenhuma inferência real |
| ONNX | `ml_onnx_run()` retorna string descritiva em vez de executar inferência |
| CUDA | Zero código, nenhum crate |
| OpenCL | Zero código, nenhum crate |
| LLMs | `llm_load_gguf/generate/chat` salvam metadados, sem geração de texto |
| Embeddings | `embed_create/encode` retornam strings descritivas |
| Unity/Unreal | Bridges salvam nomes em HashMap, sem FFI real |
| OpenGL/Vulkan/DirectX | Arquivos existem em `src/render/` mas sem `mod.rs` + `ash` não está no Cargo.toml |
| SSR/SSG/SPA/PWA | Nenhuma implementação específica |
| Web Components | Sem custom elements ou shadow DOM |
| XML/YAML/INI | Nenhum parser implementado |
| Profiler | Só PGO, sem profiler de performance ou flamegraphs |
| .NET | Gerador de código C#, não integração runtime |

---

## ✅ Confirmado como Funcional

| Feature | Onde | Notas |
|---------|------|-------|
| Passagem por referência | `tomb(&var)` | Funcional |
| Borrow Checker | `borrow_count` em `Var` | Runtime, não compile-time |
| Drivers/Kernel/Bootloaders | `src/native/system.rs` | Wrappers sysadmin (`modprobe`, `dmesg`, `grubby`, etc.) |
| PowerShell | `src/native/system.rs` | Invoca `powershell`/`pwsh` via `std::process::Command` |
| Batch | `src/native/system.rs` | Invoca `cmd.exe /c` |
| Benchmark suite | `benchmarks/` | `.aly`, `.c`, `.cpp`, `.go`, `.rs` presentes |
| Shell (bash) | `src/native/shell.rs` | Implementado |
| LLVM Backend | `src/compiler/llvm_backend.rs` | Existe e module-izado |
| C++ ABI | `src/compiler/cpp_abi.rs` | Existe |
| Metaprogramming | `src/compiler/metaprogramming.rs` | Existe |
| Cache / Incremental / Parallel | `src/compiler/cache.rs`, `incremental.rs`, `parallel.rs` | Todos existem |
| Arena Allocator | `src/compiler/arena.rs` | Existe |
| Regalloc | `src/vm/regalloc/` | Existe (4 arquivos) |
| Database PostgreSQL/SQLite/MySQL/Redis | `src/stdlib/database/` | Todos implementam `DatabaseDriver` trait |
| GUI: FLTK, GTK4, Qt, DOM | `src/native/gui_fltk.rs`, `gtk_backend.rs`, `qt_backend.rs`, `dom_backend.rs` | Implementam `GuiBackend` trait |
| Jogos 2D/3D | `src/native/game.rs` | Sprites, tilemaps, wgpu draw, 3D scene |
| IoT: MQTT, GPIO, I2C, SPI, UART | `src/native/` | `rppal` e `rumqttc` reais |
| Segurança: JWT, OAuth2, OIDC, TLS | `src/stdlib/security/` | Todos reais |
| WASM Backend | `src/native/wasm_backend.rs` | Agora declarado como módulo |

---

## 📊 Resumo

| Categoria | Qtd |
|-----------|-----|
| Items desmarcados no `todo.md` | 20 |
| Features corrigidas no `Cargo.toml` | 3 |
| Módulos ativados (`mod.rs`) | 1 |
| Caminhos corrigidos | 1 |
| **Total de correções** | **25** |