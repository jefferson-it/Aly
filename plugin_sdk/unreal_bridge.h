#ifndef ALY_UNREAL_BRIDGE_H
#define ALY_UNREAL_BRIDGE_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Aly Unreal Engine Bridge — C ABI Plugin Header
 * ===============================================
 *
 * Use this header to build a native module that Unreal Engine (C++) can
 * load as a Runtime plugin or third-party library.
 *
 * Building (as Unreal module):
 *   1. Create an Unreal Plugin with a ThirdParty dependency
 *   2. Add this header and the .c/.cpp implementation to Source/
 *   3. Call aly_bridge_init() during your module's StartupModule()
 *
 * Unreal C++ usage:
 *   #include "aly_unreal_bridge.h"
 *   // Initialize bridge on game start
 *   char* result = aly_bridge_unreal_init("mygame", "game.init");
 *   // Call Aly functions during gameplay
 *   char* res = aly_bridge_unreal_call("mygame", "player.jump", "{\"force\": 500}");
 *   aly_bridge_free_string(res);
 */

/* Maximum string length for bridge operations */
#define ALY_BRIDGE_MAX_STR 65536

/*
 * Initialize an Aly Unreal bridge instance.
 *
 *   bridge_name  – unique name for this bridge connection
 *   entry_point  – the Aly function to call on startup (e.g. "game.init")
 *
 * Returns a JSON string with status info. Caller must free with aly_bridge_free_string().
 */
char* aly_bridge_unreal_init(const char* bridge_name, const char* entry_point);

/*
 * Call an Aly function through the Unreal bridge.
 *
 *   bridge_name – bridge instance name
 *   function    – Aly function name to call
 *   args_json   – JSON string of arguments
 *
 * Returns a JSON string with the result. Caller must free with aly_bridge_free_string().
 */
char* aly_bridge_unreal_call(const char* bridge_name, const char* function, const char* args_json);

/*
 * Register a C++ function so it can be called from Aly scripts.
 *
 *   bridge_name   – bridge instance name
 *   function_name – the name Aly scripts will use to call this function
 *
 * Returns a JSON status string.
 */
char* aly_bridge_unreal_register(const char* bridge_name, const char* function_name);

/*
 * Send an event to the Unreal Engine event system.
 *
 *   event_name – name of the event (e.g. "OnPlayerDied")
 *   payload    – JSON payload with event data
 *
 * Returns a JSON status string.
 */
char* aly_bridge_unreal_send_event(const char* event_name, const char* payload);

/*
 * Free a string returned by any aly_bridge_unreal_* function.
 */
void aly_bridge_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* ALY_UNREAL_BRIDGE_H */