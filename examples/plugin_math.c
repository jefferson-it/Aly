#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../plugin_sdk/plugin.h"

static AlyPluginInfo info = {
    .api_version = ALY_PLUGIN_API_VERSION,
    .name        = "Math Extension",
    .version     = "1.0.0",
    .description = "Funções matemáticas extras para Aly",
};

AlyPluginInfo* aly_plugin_init(void) {
    return &info;
}

/* ── helper: parse an integer from args ────────────────────────────── */
static int parse_int(const char* args, int* out) {
    if (!args || !*args) return -1;
    char* end = NULL;
    long val = strtol(args, &end, 10);
    if (end == args) return -1;
    *out = (int)val;
    return 0;
}

/* ── helper: parse two integers ────────────────────────────────────── */
static int parse_two_ints(const char* args, int* a, int* b) {
    if (!args || !*args) return -1;
    char buf[256];
    strncpy(buf, args, sizeof(buf) - 1);
    buf[sizeof(buf) - 1] = '\0';
    char* comma = strchr(buf, ',');
    if (!comma) return -1;
    *comma = '\0';
    char* end = NULL;
    long va = strtol(buf, &end, 10);
    if (end == buf) return -1;
    long vb = strtol(comma + 1, &end, 10);
    if (end == comma + 1) return -1;
    *a = (int)va;
    *b = (int)vb;
    return 0;
}

/* ── helper: allocate a result string ──────────────────────────────── */
static char* result_str(const char* s) {
    char* p = (char*)malloc(strlen(s) + 1);
    if (p) strcpy(p, s);
    return p;
}

static char* result_int(int n) {
    char buf[64];
    snprintf(buf, sizeof(buf), "%d", n);
    return result_str(buf);
}

/* ── exported functions ────────────────────────────────────────────── */

static char* cmd_fibonacci(const char* args) {
    int n;
    if (parse_int(args, &n) != 0 || n < 0) return NULL;
    if (n == 0) return result_int(0);
    if (n == 1) return result_int(1);
    int a = 0, b = 1;
    for (int i = 2; i <= n; i++) {
        int c = a + b;
        a = b;
        b = c;
    }
    return result_int(b);
}

static char* cmd_factorial(const char* args) {
    int n;
    if (parse_int(args, &n) != 0 || n < 0) return NULL;
    if (n <= 1) return result_int(1);
    long long r = 1;
    for (int i = 2; i <= n; i++) r *= i;
    char buf[64];
    snprintf(buf, sizeof(buf), "%lld", r);
    return result_str(buf);
}

static char* cmd_add(const char* args) {
    int a, b;
    if (parse_two_ints(args, &a, &b) != 0) return NULL;
    return result_int(a + b);
}

static char* cmd_multiply(const char* args) {
    int a, b;
    if (parse_two_ints(args, &a, &b) != 0) return NULL;
    return result_int(a * b);
}

/* ── dispatcher ───────────────────────────────────────────────────── */

char* aly_plugin_call(const char* func_name, const char* args) {
    if (strcmp(func_name, "fibonacci") == 0)  return cmd_fibonacci(args);
    if (strcmp(func_name, "factorial") == 0)  return cmd_factorial(args);
    if (strcmp(func_name, "add")      == 0)  return cmd_add(args);
    if (strcmp(func_name, "multiply") == 0)  return cmd_multiply(args);
    return NULL; /* unknown function */
}

void aly_plugin_free_string(char* ptr) {
    free(ptr);
}

char* aly_plugin_functions(void) {
    /* Comma-separated list of exported function names */
    char* list = strdup("fibonacci,factorial,add,multiply");
    return list;
}
