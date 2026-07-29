#include "runtime_aly.h"

// ---- Aly runtime wrappers ----
static void print(aly_value_t v) { aly_print(v); }
static void print_raw(aly_value_t v) { aly_print_raw(v); }
static aly_value_t input(const char* prompt) { return aly_input(prompt); }
// ---- end wrappers ----

int main(int argc, char** argv) {
    aly_init(argc, argv);
    aly_value_t i = aly_int(0);
    aly_value_t sum = aly_int(0);
    while (aly_compare(i, "<", aly_int(5))) {
        sum = aly_add(sum, i);
        i = aly_add(i, aly_int(1));
    }
    print(({ char _buf_0[512]; snprintf(_buf_0, sizeof(_buf_0), "Conditional loop sum (expected 10): %s", aly_to_str(sum)); aly_string(_buf_0); }));
    print(({ char _buf_1[512]; snprintf(_buf_1, sizeof(_buf_1), "Conditional loop index (expected 5): %s", aly_to_str(i)); aly_string(_buf_1); }));
    aly_value_t sum2 = aly_int(0);
    aly_value_t j = aly_int(0);
for (j = aly_int(1); aly_compare(j, "<=", aly_int(5)); j = aly_add(j, aly_int(1))) {
        sum2 = aly_add(sum2, j);
    }
    print(({ char _buf_2[512]; snprintf(_buf_2, sizeof(_buf_2), "Three-part loop sum2 (expected 15): %s", aly_to_str(sum2)); aly_string(_buf_2); }));
    aly_value_t k = aly_int(0);
    aly_value_t even_count = aly_int(0);
    while (aly_compare(k, "<", aly_int(5))) {
        if (aly_compare(aly_mod(k, aly_int(2)), "==", aly_int(0))) {
            even_count = aly_add(even_count, aly_int(1));
        }
        k = aly_add(k, aly_int(1));
    }
    print(({ char _buf_3[512]; snprintf(_buf_3, sizeof(_buf_3), "Nested if in loop even_count (expected 3): %s", aly_to_str(even_count)); aly_string(_buf_3); }));
    aly_cleanup();
    return 0;
}

