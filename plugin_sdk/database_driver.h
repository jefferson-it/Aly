#ifndef ALY_DATABASE_DRIVER_H
#define ALY_DATABASE_DRIVER_H

#ifdef __cplusplus
extern "C" {
#endif

#define ALY_DATABASE_DRIVER_API_VERSION 1

/*
 * Result type for database operations.
 * If error_msg is NULL, the operation succeeded.
 */
typedef struct {
    int success;           /* 1 = success, 0 = error */
    const char* error_msg; /* Error message (NULL if success), freed by driver */
    const char* result;    /* Result string (JSON-encoded), freed by driver */
} AlyDbResult;

/*
 * Database driver instance.
 * Returned by aly_db_driver_connect(), used for all subsequent operations.
 * Opaque from the runtime's perspective.
 */
typedef struct {
    void* handle;          /* Opaque driver handle, allocated by connect */
} AlyDbConnection;

/*
 * Driver metadata returned by aly_db_driver_init().
 * All string pointers must remain valid for the lifetime of the plugin.
 */
typedef struct {
    int     api_version;   /* Must be ALY_DATABASE_DRIVER_API_VERSION */
    const char* name;      /* Driver name (e.g. "mongodb", "cassandra") */
    const char* version;   /* Driver version string */
    const char* description;
} AlyDatabaseDriverInfo;

/*
 * ── Required exports ──────────────────────────────────────────────────
 */

/*
 * Return a pointer to a static AlyDatabaseDriverInfo struct.
 * Never return NULL.
 */
AlyDatabaseDriverInfo* aly_db_driver_init(void);

/*
 * Connect to a database.
 *
 *   uri     – connection URI (e.g. "mongodb://localhost:27017/mydb")
 *   options – JSON-encoded options string (may be NULL or "{}")
 *
 * Returns a AlyDbConnection. On error, set connection.handle to NULL
 * and populate the error details.
 */
AlyDbConnection aly_db_driver_connect(const char* uri, const char* options);

/*
 * Execute a query (expects result set).
 *
 *   conn – connection returned by aly_db_driver_connect
 *   sql  – query string
 *
 * Returns AlyDbResult with result set as JSON array.
 */
AlyDbResult aly_db_driver_query(AlyDbConnection conn, const char* sql);

/*
 * Execute a command (no result set expected).
 *
 *   conn – connection returned by aly_db_driver_connect
 *   sql  – command string
 *
 * Returns AlyDbResult with affected rows count as JSON number.
 */
AlyDbResult aly_db_driver_execute(AlyDbConnection conn, const char* sql);

/*
 * Close the connection and free all resources.
 */
void aly_db_driver_close(AlyDbConnection conn);

/*
 * Free a string previously returned by any AlyDbResult field.
 * If ptr is NULL, the call is a no-op.
 */
void aly_db_driver_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* ALY_DATABASE_DRIVER_H */
