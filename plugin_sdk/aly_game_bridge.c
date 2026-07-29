/*
 * Aly Game Engine Bridge — C Reference Implementation
 * ====================================================
 * Reference implementation for Unity/Unreal bridge.
 * Loads libaly_rust.so via dlopen and forwards calls to Rust functions.
 *
 * Build:
 *   gcc -shared -fPIC -o aly_game_bridge.so aly_game_bridge.c -ldl
 */

#include "unity_bridge.h"
#include "unreal_bridge.h"
#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void* aly_lib = NULL;
typedef char* (*AlyEvalFn)(const char*);
typedef void  (*AlyFreeStrFn)(char*);
static AlyEvalFn aly_eval = NULL;
static AlyFreeStrFn aly_free_str = NULL;

static int load_aly(void) {
    if (aly_lib) return 0;
    const char* paths[] = {
        "./libaly_rust.so", "./target/release/libaly_rust.so",
        "./target/debug/libaly_rust.so", "/usr/local/lib/libaly_rust.so", NULL
    };
    for (int i = 0; paths[i]; i++) {
        aly_lib = dlopen(paths[i], RTLD_NOW | RTLD_GLOBAL);
        if (aly_lib) break;
    }
    if (!aly_lib) return -1;
    aly_eval = (AlyEvalFn)dlsym(aly_lib, "aly_eval");
    aly_free_str = (AlyFreeStrFn)dlsym(aly_lib, "aly_free_string");
    if (!aly_eval || !aly_free_str) { dlclose(aly_lib); aly_lib = NULL; return -1; }
    return 0;
}

static char* json_result(int ok, const char* msg, const char* data) {
    char buf[8192];
    snprintf(buf, sizeof(buf),
        "{\"success\":%s,\"message\":\"%s\",\"data\":\"%s\"}",
        ok ? "true" : "false", msg, data);
    return strdup(buf);
}

static char* call_aly(const char* fmt, const char* a1, const char* a2, const char* a3) {
    if (!aly_lib) return json_result(0, "Aly not loaded. Call init first.", "");
    char code[4096];
    snprintf(code, sizeof(code), fmt, a1, a2, a3);
    char* r = aly_eval(code);
    char* j = json_result(1, "ok", r ? r : "");
    if (r) aly_free_str(r);
    return j;
}

/* ── Unity API ──────────────────────────────────────────────────── */

char* aly_bridge_init(const char* name, const char* ep) {
    if (load_aly() != 0) return json_result(0, "Cannot load Aly runtime", "");
    return call_aly("game.unity_init(\"%s\",\"%s\")", name, ep, "");
}
char* aly_bridge_call(const char* n, const char* fn, const char* a) {
    return call_aly("game.unity_call(\"%s\",\"%s\",\"%s\")", n, fn, a);
}
char* aly_bridge_send_message(const char* go, const char* m, const char* p) {
    return call_aly("game.unity_send_message(\"%s\",\"%s\",\"%s\")", go, m, p);
}
char* aly_bridge_register_external(const char* n, const char* fn) {
    return call_aly("game.unity_register_external(\"%s\",\"%s\")", n, fn, "");
}

/* ── Unreal API ─────────────────────────────────────────────────── */

char* aly_bridge_unreal_init(const char* n, const char* ep) {
    if (load_aly() != 0) return json_result(0, "Cannot load Aly runtime", "");
    return call_aly("game.unreal_init(\"%s\",\"%s\")", n, ep, "");
}
char* aly_bridge_unreal_call(const char* n, const char* fn, const char* a) {
    return call_aly("game.unreal_call(\"%s\",\"%s\",\"%s\")", n, fn, a);
}
char* aly_bridge_unreal_register(const char* n, const char* fn) {
    return call_aly("game.unreal_register_function(\"%s\",\"%s\")", n, fn, "");
}
char* aly_bridge_unreal_send_event(const char* e, const char* p) {
    return call_aly("game.unreal_send_event(\"%s\",\"%s\")", e, p, "");
}

void aly_bridge_free_string(char* ptr) { if (ptr) free(ptr); }

__attribute__((destructor))
static void cleanup(void) { if (aly_lib) { dlclose(aly_lib); aly_lib = NULL; } }