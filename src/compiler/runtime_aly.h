#ifndef RUNTIME_ALY_H
#define RUNTIME_ALY_H

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <ctype.h>
#include <time.h>
#include <stdarg.h>

typedef enum {
    ALY_INT,
    ALY_FLOAT,
    ALY_BOOL,
    ALY_STRING,
    ALY_ARRAY,
    ALY_OBJECT,
    ALY_NONE,
    ALY_FUNCTION
} aly_type_t;

typedef struct aly_object aly_object_t;
typedef struct aly_array aly_array_t;

typedef struct {
    aly_type_t type;
    union {
        long long int_val;
        double float_val;
        int bool_val;
        char* str_val;
        aly_array_t* array_val;
        aly_object_t* object_val;
    } as;
    char* type_tag;
} aly_value_t;

struct aly_object_entry {
    char* key;
    aly_value_t value;
    int used;
};

struct aly_object {
    aly_object_entry_t* entries;
    int size;
    int capacity;
};

struct aly_array {
    aly_value_t* items;
    int size;
    int capacity;
};

typedef void (*aly_prop_func)(void);

struct aly_object_vtable {
    aly_prop_func props[64];
};

aly_value_t aly_int(long long v) { aly_value_t r; r.type = ALY_INT; r.as.int_val = v; r.type_tag = NULL; return r; }
aly_value_t aly_float(double v) { aly_value_t r; r.type = ALY_FLOAT; r.as.float_val = v; r.type_tag = NULL; return r; }
aly_value_t aly_bool(int v) { aly_value_t r; r.type = ALY_BOOL; r.as.bool_val = v; r.type_tag = NULL; return r; }
aly_value_t aly_none(void) { aly_value_t r; r.type = ALY_NONE; r.as.int_val = 0; r.type_tag = NULL; return r; }

aly_value_t aly_string(const char* s) {
    aly_value_t r;
    r.type = ALY_STRING;
    r.as.str_val = strdup(s);
    r.type_tag = NULL;
    return r;
}

aly_value_t aly_string_alloc(int len) {
    aly_value_t r;
    r.type = ALY_STRING;
    r.as.str_val = (char*)calloc(len + 1, 1);
    r.type_tag = NULL;
    return r;
}

void aly_free(aly_value_t v) {
    switch (v.type) {
        case ALY_STRING: free(v.as.str_val); break;
        case ALY_ARRAY:
            if (v.as.array_val) {
                for (int i = 0; i < v.as.array_val->size; i++) aly_free(v.as.array_val->items[i]);
                free(v.as.array_val->items);
                free(v.as.array_val);
            }
            break;
        case ALY_OBJECT:
            if (v.as.object_val) {
                for (int i = 0; i < v.as.object_val->capacity; i++) {
                    if (v.as.object_val->entries[i].used) {
                        free(v.as.object_val->entries[i].key);
                        aly_free(v.as.object_val->entries[i].value);
                    }
                }
                free(v.as.object_val->entries);
                free(v.as.object_val);
            }
            break;
        default: break;
    }
    if (v.type_tag) free(v.type_tag);
}

aly_value_t aly_clone(aly_value_t v) {
    switch (v.type) {
        case ALY_STRING: return aly_string(v.as.str_val);
        case ALY_INT: return aly_int(v.as.int_val);
        case ALY_FLOAT: return aly_float(v.as.float_val);
        case ALY_BOOL: return aly_bool(v.as.bool_val);
        default: return v;
    }
}

int aly_is_truthy(aly_value_t v) {
    switch (v.type) {
        case ALY_BOOL: return v.as.bool_val;
        case ALY_INT: return v.as.int_val != 0;
        case ALY_FLOAT: return v.as.float_val != 0.0;
        case ALY_STRING: return v.as.str_val && v.as.str_val[0] != '\0';
        case ALY_ARRAY: return v.as.array_val && v.as.array_val->size > 0;
        case ALY_OBJECT: return v.as.object_val && v.as.object_val->size > 0;
        case ALY_NONE: return 0;
        default: return 0;
    }
}

const char* aly_type_name(aly_value_t v) {
    switch (v.type) {
        case ALY_INT: return "int";
        case ALY_FLOAT: return "float";
        case ALY_BOOL: return "bool";
        case ALY_STRING: return "string";
        case ALY_ARRAY: return "array";
        case ALY_OBJECT: return "object";
        case ALY_NONE: return "None";
        case ALY_FUNCTION: return "function";
        default: return "unknown";
    }
}

aly_value_t aly_to_int(aly_value_t v) {
    switch (v.type) {
        case ALY_INT: return v;
        case ALY_FLOAT: return aly_int((long long)v.as.float_val);
        case ALY_BOOL: return aly_int(v.as.bool_val ? 1 : 0);
        case ALY_STRING: return aly_int(atoll(v.as.str_val));
        default: return aly_int(0);
    }
}

aly_value_t aly_to_float(aly_value_t v) {
    switch (v.type) {
        case ALY_INT: return aly_float((double)v.as.int_val);
        case ALY_FLOAT: return v;
        case ALY_STRING: return aly_float(atof(v.as.str_val));
        default: return aly_float(0.0);
    }
}

aly_value_t aly_to_str(aly_value_t v) {
    char buf[512];
    switch (v.type) {
        case ALY_INT: snprintf(buf, sizeof(buf), "%lld", v.as.int_val); return aly_string(buf);
        case ALY_FLOAT: snprintf(buf, sizeof(buf), "%g", v.as.float_val); return aly_string(buf);
        case ALY_BOOL: return aly_string(v.as.bool_val ? "true" : "false");
        case ALY_STRING: return aly_clone(v);
        case ALY_NONE: return aly_string("None");
        case ALY_ARRAY: {
            aly_array_t* arr = v.as.array_val;
            if (!arr || arr->size == 0) return aly_string("[]");
            char* out = (char*)calloc(1, 1024);
            strcat(out, "[");
            for (int i = 0; i < arr->size; i++) {
                aly_value_t s = aly_to_str(arr->items[i]);
                if (i > 0) strcat(out, ", ");
                strcat(out, s.as.str_val);
                aly_free(s);
            }
            strcat(out, "]");
            aly_value_t result = aly_string(out);
            free(out);
            return result;
        }
        case ALY_OBJECT: {
            aly_object_t* obj = v.as.object_val;
            if (!obj || obj->size == 0) return aly_string("{}");
            char* out = (char*)calloc(1, 1024);
            strcat(out, "{");
            int first = 1;
            for (int i = 0; i < obj->capacity; i++) {
                if (obj->entries[i].used) {
                    if (!first) strcat(out, ", ");
                    first = 0;
                    strcat(out, obj->entries[i].key);
                    strcat(out, ": ");
                    aly_value_t s = aly_to_str(obj->entries[i].value);
                    strcat(out, s.as.str_val);
                    aly_free(s);
                }
            }
            strcat(out, "}");
            aly_value_t result = aly_string(out);
            free(out);
            return result;
        }
        default: return aly_string("None");
    }
}

aly_value_t aly_add(aly_value_t a, aly_value_t b) {
    if (a.type == ALY_STRING || b.type == ALY_STRING) {
        aly_value_t sa = aly_to_str(a);
        aly_value_t sb = aly_to_str(b);
        int len = strlen(sa.as.str_val) + strlen(sb.as.str_val) + 1;
        char* buf = (char*)malloc(len);
        snprintf(buf, len, "%s%s", sa.as.str_val, sb.as.str_val);
        aly_free(sa);
        aly_free(sb);
        aly_value_t r = aly_string(buf);
        free(buf);
        return r;
    }
    if (a.type == ALY_FLOAT || b.type == ALY_FLOAT)
        return aly_float(aly_to_float(a).as.float_val + aly_to_float(b).as.float_val);
    return aly_int(aly_to_int(a).as.int_val + aly_to_int(b).as.int_val);
}

aly_value_t aly_sub(aly_value_t a, aly_value_t b) {
    if (a.type == ALY_FLOAT || b.type == ALY_FLOAT)
        return aly_float(aly_to_float(a).as.float_val - aly_to_float(b).as.float_val);
    return aly_int(aly_to_int(a).as.int_val - aly_to_int(b).as.int_val);
}

aly_value_t aly_mul(aly_value_t a, aly_value_t b) {
    if (a.type == ALY_STRING && b.type == ALY_INT) {
        int n = b.as.int_val;
        aly_value_t s = aly_to_str(a);
        int len = strlen(s.as.str_val) * n + 1;
        char* buf = (char*)malloc(len);
        buf[0] = '\0';
        for (int i = 0; i < n; i++) strcat(buf, s.as.str_val);
        aly_free(s);
        aly_value_t r = aly_string(buf);
        free(buf);
        return r;
    }
    if (a.type == ALY_FLOAT || b.type == ALY_FLOAT)
        return aly_float(aly_to_float(a).as.float_val * aly_to_float(b).as.float_val);
    return aly_int(aly_to_int(a).as.int_val * aly_to_int(b).as.int_val);
}

aly_value_t aly_div(aly_value_t a, aly_value_t b) {
    double da = aly_to_float(a).as.float_val;
    double db = aly_to_float(b).as.float_val;
    if (db == 0.0) { fprintf(stderr, "Error: Division by zero\n"); exit(1); }
    return aly_float(da / db);
}

aly_value_t aly_mod(aly_value_t a, aly_value_t b) {
    if (a.type == ALY_INT && b.type == ALY_INT) {
        if (b.as.int_val == 0) { fprintf(stderr, "Error: Modulo by zero\n"); exit(1); }
        return aly_int(a.as.int_val % b.as.int_val);
    }
    return aly_float(fmod(aly_to_float(a).as.float_val, aly_to_float(b).as.float_val));
}

aly_value_t aly_neg(aly_value_t v) {
    if (v.type == ALY_FLOAT) return aly_float(-v.as.float_val);
    return aly_int(-aly_to_int(v).as.int_val);
}

aly_value_t aly_not(aly_value_t v) {
    return aly_bool(!aly_is_truthy(v));
}

aly_value_t aly_and(aly_value_t a, aly_value_t b) {
    return aly_bool(aly_is_truthy(a) && aly_is_truthy(b));
}

aly_value_t aly_or(aly_value_t a, aly_value_t b) {
    return aly_bool(aly_is_truthy(a) || aly_is_truthy(b));
}

aly_value_t aly_xor(aly_value_t a, aly_value_t b) {
    return aly_bool(aly_is_truthy(a) ^ aly_is_truthy(b));
}

aly_value_t aly_percent(aly_value_t v) {
    if (v.type == ALY_FLOAT) return aly_float(v.as.float_val / 100.0);
    return aly_float((double)aly_to_int(v).as.int_val / 100.0);
}

int aly_compare_int(aly_value_t a, const char* op, aly_value_t b) {
    long long va = aly_to_int(a).as.int_val;
    long long vb = aly_to_int(b).as.int_val;
    if (strcmp(op, "==") == 0) return va == vb;
    if (strcmp(op, "!=") == 0) return va != vb;
    if (strcmp(op, "<") == 0) return va < vb;
    if (strcmp(op, "<=") == 0) return va <= vb;
    if (strcmp(op, ">") == 0) return va > vb;
    if (strcmp(op, ">=") == 0) return va >= vb;
    return 0;
}

aly_value_t aly_compare(aly_value_t a, const char* op, aly_value_t b) {
    if (a.type == ALY_STRING || b.type == ALY_STRING) {
        aly_value_t sa = aly_to_str(a);
        aly_value_t sb = aly_to_str(b);
        int cmp = strcmp(sa.as.str_val, sb.as.str_val);
        int result = 0;
        if (strcmp(op, "==") == 0) result = (cmp == 0);
        else if (strcmp(op, "!=") == 0) result = (cmp != 0);
        else if (strcmp(op, "<") == 0) result = (cmp < 0);
        else if (strcmp(op, "<=") == 0) result = (cmp <= 0);
        else if (strcmp(op, ">") == 0) result = (cmp > 0);
        else if (strcmp(op, ">=") == 0) result = (cmp >= 0);
        aly_free(sa);
        aly_free(sb);
        return aly_bool(result);
    }
    if (a.type == ALY_FLOAT || b.type == ALY_FLOAT) {
        double va = aly_to_float(a).as.float_val;
        double vb = aly_to_float(b).as.float_val;
        int result = 0;
        if (strcmp(op, "==") == 0) result = (va == vb);
        else if (strcmp(op, "!=") == 0) result = (va != vb);
        else if (strcmp(op, "<") == 0) result = (va < vb);
        else if (strcmp(op, "<=") == 0) result = (va <= vb);
        else if (strcmp(op, ">") == 0) result = (va > vb);
        else if (strcmp(op, ">=") == 0) result = (va >= vb);
        return aly_bool(result);
    }
    return aly_bool(aly_compare_int(a, op, b));
}

int aly_eq(aly_value_t a, aly_value_t b) {
    aly_value_t r = aly_compare(a, "==", b);
    return r.as.bool_val;
}

int aly_gte(aly_value_t a, aly_value_t b) {
    aly_value_t r = aly_compare(a, ">=", b);
    return r.as.bool_val;
}

int aly_lte(aly_value_t a, aly_value_t b) {
    aly_value_t r = aly_compare(a, "<=", b);
    return r.as.bool_val;
}

void aly_print(aly_value_t v) {
    if (v.type == ALY_STRING) {
        printf("%s\n", v.as.str_val);
    } else {
        aly_value_t s = aly_to_str(v);
        printf("%s\n", s.as.str_val);
        aly_free(s);
    }
}

void aly_print_raw(aly_value_t v) {
    if (v.type == ALY_STRING) {
        printf("%s", v.as.str_val);
    } else {
        aly_value_t s = aly_to_str(v);
        printf("%s", s.as.str_val);
        aly_free(s);
    }
}

aly_value_t aly_input(const char* prompt) {
    if (prompt) printf("%s", prompt);
    char buf[4096];
    if (fgets(buf, sizeof(buf), stdin)) {
        buf[strcspn(buf, "\n")] = 0;
        return aly_string(buf);
    }
    return aly_string("");
}

aly_value_t aly_tomb(aly_value_t* v) {
    return *v;
}

aly_array_t* aly_array_new(int capacity) {
    aly_array_t* arr = (aly_array_t*)calloc(1, sizeof(aly_array_t));
    arr->capacity = capacity > 0 ? capacity : 8;
    arr->items = (aly_value_t*)calloc(arr->capacity, sizeof(aly_value_t));
    arr->size = 0;
    return arr;
}

aly_value_t aly_array_init(int count, aly_value_t* items) {
    aly_array_t* arr = aly_array_new(count);
    for (int i = 0; i < count; i++) {
        arr->items[i] = aly_clone(items[i]);
        arr->size++;
    }
    aly_value_t r;
    r.type = ALY_ARRAY;
    r.as.array_val = arr;
    r.type_tag = NULL;
    return r;
}

aly_value_t aly_array_get(aly_value_t arr, aly_value_t idx) {
    if (arr.type != ALY_ARRAY || !arr.as.array_val) return aly_none();
    int i = (int)aly_to_int(idx).as.int_val;
    if (i < 0 || i >= arr.as.array_val->size) return aly_none();
    return aly_clone(arr.as.array_val->items[i]);
}

void aly_array_push(aly_value_t arr, aly_value_t val) {
    if (arr.type != ALY_ARRAY || !arr.as.array_val) return;
    aly_array_t* a = arr.as.array_val;
    if (a->size >= a->capacity) {
        a->capacity *= 2;
        a->items = (aly_value_t*)realloc(a->items, a->capacity * sizeof(aly_value_t));
    }
    a->items[a->size++] = aly_clone(val);
}

aly_object_t* aly_object_new(void) {
    aly_object_t* obj = (aly_object_t*)calloc(1, sizeof(aly_object_t));
    obj->capacity = 32;
    obj->entries = (aly_object_entry_t*)calloc(obj->capacity, sizeof(aly_object_entry_t));
    obj->size = 0;
    return obj;
}

static unsigned int aly_hash(const char* key) {
    unsigned int h = 5381;
    while (*key) h = h * 33 + (unsigned char)*key++;
    return h;
}

void aly_object_set(aly_value_t obj, const char* key, aly_value_t value) {
    if (obj.type != ALY_OBJECT || !obj.as.object_val) return;
    aly_object_t* o = obj.as.object_val;
    unsigned int idx = aly_hash(key) % o->capacity;

    for (int i = 0; i < o->capacity; i++) {
        unsigned int probe = (idx + i) % o->capacity;
        if (o->entries[probe].used && strcmp(o->entries[probe].key, key) == 0) {
            aly_free(o->entries[probe].value);
            o->entries[probe].value = aly_clone(value);
            return;
        }
        if (!o->entries[probe].used) {
            o->entries[probe].key = strdup(key);
            o->entries[probe].value = aly_clone(value);
            o->entries[probe].used = 1;
            o->size++;
            return;
        }
    }
}

aly_value_t aly_object_get(aly_value_t obj, const char* key) {
    if (obj.type != ALY_OBJECT || !obj.as.object_val) return aly_none();
    aly_object_t* o = obj.as.object_val;
    unsigned int idx = aly_hash(key) % o->capacity;

    for (int i = 0; i < o->capacity; i++) {
        unsigned int probe = (idx + i) % o->capacity;
        if (!o->entries[probe].used) return aly_none();
        if (strcmp(o->entries[probe].key, key) == 0)
            return aly_clone(o->entries[probe].value);
    }
    return aly_none();
}

int aly_len(aly_value_t v) {
    switch (v.type) {
        case ALY_STRING: return v.as.str_val ? (int)strlen(v.as.str_val) : 0;
        case ALY_ARRAY: return v.as.array_val ? v.as.array_val->size : 0;
        case ALY_OBJECT: return v.as.object_val ? v.as.object_val->size : 0;
        default: return 0;
    }
}

aly_value_t aly_type_of(aly_value_t v) {
    return aly_string(aly_type_name(v));
}

/* Math functions */
aly_value_t aly_pow(aly_value_t base, aly_value_t exp) {
    return aly_float(pow(aly_to_float(base).as.float_val, aly_to_float(exp).as.float_val));
}

aly_value_t aly_sqrt(aly_value_t v) {
    return aly_float(sqrt(aly_to_float(v).as.float_val));
}

aly_value_t aly_round(aly_value_t v) {
    return aly_int((long long)round(aly_to_float(v).as.float_val));
}

aly_value_t aly_round_up(aly_value_t v) {
    return aly_int((long long)ceil(aly_to_float(v).as.float_val));
}

aly_value_t aly_round_down(aly_value_t v) {
    return aly_int((long long)floor(aly_to_float(v).as.float_val));
}

aly_value_t aly_abs(aly_value_t v) {
    if (v.type == ALY_FLOAT) return aly_float(fabs(v.as.float_val));
    long long val = aly_to_int(v).as.int_val;
    return aly_int(val < 0 ? -val : val);
}

aly_value_t aly_random(void) {
    return aly_float((double)rand() / (double)RAND_MAX);
}

aly_value_t aly_sin(aly_value_t v) { return aly_float(sin(aly_to_float(v).as.float_val)); }
aly_value_t aly_cos(aly_value_t v) { return aly_float(cos(aly_to_float(v).as.float_val)); }
aly_value_t aly_tan(aly_value_t v) { return aly_float(tan(aly_to_float(v).as.float_val)); }
aly_value_t aly_log(aly_value_t v) { return aly_float(log10(aly_to_float(v).as.float_val)); }
aly_value_t aly_ln(aly_value_t v) { return aly_float(log(aly_to_float(v).as.float_val)); }
aly_value_t aly_min(aly_value_t a, aly_value_t b) {
    return aly_to_float(a).as.float_val < aly_to_float(b).as.float_val ? a : b;
}
aly_value_t aly_max(aly_value_t a, aly_value_t b) {
    return aly_to_float(a).as.float_val > aly_to_float(b).as.float_val ? a : b;
}

/* String functions */
aly_value_t aly_str_upper(aly_value_t v) {
    aly_value_t s = aly_to_str(v);
    for (char* p = s.as.str_val; *p; p++) *p = toupper(*p);
    return s;
}

aly_value_t aly_str_lower(aly_value_t v) {
    aly_value_t s = aly_to_str(v);
    for (char* p = s.as.str_val; *p; p++) *p = tolower(*p);
    return s;
}

aly_value_t aly_str_trim(aly_value_t v) {
    aly_value_t s = aly_to_str(v);
    char* start = s.as.str_val;
    while (isspace(*start)) start++;
    char* end = start + strlen(start) - 1;
    while (end > start && isspace(*end)) end--;
    *(end + 1) = 0;
    aly_value_t result = aly_string(start);
    aly_free(s);
    return result;
}

int aly_str_contains(aly_value_t haystack, aly_value_t needle) {
    aly_value_t h = aly_to_str(haystack);
    aly_value_t n = aly_to_str(needle);
    int result = (strstr(h.as.str_val, n.as.str_val) != NULL);
    aly_free(h);
    aly_free(n);
    return result;
}

/* File system functions */
aly_value_t aly_fs_read(aly_value_t path) {
    aly_value_t p = aly_to_str(path);
    FILE* f = fopen(p.as.str_val, "r");
    aly_free(p);
    if (!f) { fprintf(stderr, "Error: Cannot open file\n"); return aly_string(""); }
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    char* buf = (char*)malloc(len + 1);
    fread(buf, 1, len, f);
    buf[len] = 0;
    fclose(f);
    aly_value_t result = aly_string(buf);
    free(buf);
    return result;
}

void aly_fs_write(aly_value_t path, aly_value_t content) {
    aly_value_t p = aly_to_str(path);
    aly_value_t c = aly_to_str(content);
    FILE* f = fopen(p.as.str_val, "w");
    if (f) { fprintf(f, "%s", c.as.str_val); fclose(f); }
    aly_free(p);
    aly_free(c);
}

void aly_fs_append(aly_value_t path, aly_value_t content) {
    aly_value_t p = aly_to_str(path);
    aly_value_t c = aly_to_str(content);
    FILE* f = fopen(p.as.str_val, "a");
    if (f) { fprintf(f, "%s", c.as.str_val); fclose(f); }
    aly_free(p);
    aly_free(c);
}

int aly_fs_exists(aly_value_t path) {
    aly_value_t p = aly_to_str(path);
    FILE* f = fopen(p.as.str_val, "r");
    aly_free(p);
    if (f) { fclose(f); return 1; }
    return 0;
}

/* Ref (tomb) */
void aly_ref(aly_value_t v) { (void)v; }

/* Global state */
static int _aly_argc = 0;
static char** _aly_argv = NULL;

void aly_init(int argc, char** argv) {
    _aly_argc = argc;
    _aly_argv = argv;
    srand((unsigned)time(NULL));
}

void aly_cleanup(void) { }

aly_value_t aly_sys_args(void) {
    aly_array_t* arr = aly_array_new(_aly_argc);
    for (int i = 0; i < _aly_argc; i++) {
        arr->items[i] = aly_string(_aly_argv[i]);
        arr->size++;
    }
    aly_value_t r;
    r.type = ALY_ARRAY;
    r.as.array_val = arr;
    r.type_tag = NULL;
    return r;
}

#endif
