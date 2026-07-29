#ifndef ALY_UNITY_BRIDGE_H
#define ALY_UNITY_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Aly Unity Bridge — C ABI Plugin Header
 * =======================================
 *
 * Use this header to build a native plugin that Unity (C#) can load
 * via P/Invoke or the Unity Native Plugin interface.
 *
 * Building:
 *   gcc -shared -o aly_unity_bridge.so aly_unity_bridge.c -ldl
 *
 * Unity C# usage (DllImport):
 *   [DllImport("aly_unity_bridge")]
 *   private static extern System.IntPtr aly_bridge_init(string bridgeName, string entryPoint);
 *
 *   [DllImport("aly_unity_bridge")]
 *   private static extern System.IntPtr aly_bridge_call(string bridgeName, string function, string argsJson);
 */

/* Maximum string length for bridge operations */
#define ALY_BRIDGE_MAX_STR 65536

/*
 * Initialize an Aly bridge instance.
 *
 *   bridge_name  – unique name for this bridge connection
 *   entry_point  – the Aly function to call on startup (e.g. "game.init")
 *
 * Returns a JSON string with status info. Caller must free with aly_bridge_free_string().
 */
char* aly_bridge_init(const char* bridge_name, const char* entry_point);

/*
 * Call an Aly function through the bridge.
 *
 *   bridge_name – bridge instance name (from aly_bridge_init)
 *   function    – Aly function name to call
 *   args_json   – JSON string of arguments (e.g. '{"x": 10, "y": 20}')
 *
 * Returns a JSON string with the result. Caller must free with aly_bridge_free_string().
 */
char* aly_bridge_call(const char* bridge_name, const char* function, const char* args_json);

/*
 * Send a message to a Unity GameObject.
 *
 *   game_object – name of the Unity GameObject
 *   method      – method name to call on the GameObject
 *   parameter   – string parameter to pass to the method
 *
 * Returns a JSON status string.
 */
char* aly_bridge_send_message(const char* game_object, const char* method, const char* parameter);

/*
 * Register an external Unity function so it can be called from Aly scripts.
 *
 *   bridge_name   – bridge instance name
 *   function_name – the name Aly scripts will use to call this function
 *
 * Returns a JSON status string.
 */
char* aly_bridge_register_external(const char* bridge_name, const char* function_name);

/*
 * Free a string returned by any aly_bridge_* function.
 */
void aly_bridge_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* ALY_UNITY_BRIDGE_H */