#ifndef ALY_PLUGIN_H
#define ALY_PLUGIN_H

#ifdef __cplusplus
extern "C" {
#endif

#define ALY_PLUGIN_API_VERSION 1

/*
 * Plugin metadata returned by aly_plugin_init().
 * All string pointers must remain valid for the lifetime of the plugin
 * (use static strings or allocate once).
 */
typedef struct {
    int             api_version;   /* Must be ALY_PLUGIN_API_VERSION */
    const char*     name;          /* Plugin name (human-readable)    */
    const char*     version;       /* Plugin version string           */
    const char*     description;   /* Short description               */
} AlyPluginInfo;

/*
 * ── Required exports ───────────────────────────────────────────────────
 * Every Aly plugin shared library MUST export these three symbols.
 */

/*
 * Return a pointer to a static AlyPluginInfo struct.
 * The pointer must remain valid for the entire lifetime of the plugin.
 * Never return NULL.
 */
AlyPluginInfo* aly_plugin_init(void);

/*
 * Call a function inside this plugin.
 *
 *   func_name  – the name of the function to invoke (UTF-8, null-terminated)
 *   args       – function arguments as a single string (comma-separated
 *                values, same format as Aly native functions)
 *
 * Returns a null-terminated UTF-8 string that the caller will free with
 * aly_plugin_free_string(). Return NULL to signal an error (the runtime
 * will treat the result as "None").
 */
char* aly_plugin_call(const char* func_name, const char* args);

/*
 * Free a string previously returned by aly_plugin_call().
 * If ptr is NULL the call is a no-op.
 */
void aly_plugin_free_string(char* ptr);

/*
 * ── Optional exports ───────────────────────────────────────────────────
 */

/*
 * Return a comma-separated list of function names exported by this plugin.
 * The returned string will be freed with aly_plugin_free_string().
 *
 * If this function is not exported, the runtime will NOT register
 * individual namespace.funcname variables — users must use
 * the generic call mechanism instead.
 */
char* aly_plugin_functions(void);

#ifdef __cplusplus
}
#endif

#endif /* ALY_PLUGIN_H */
