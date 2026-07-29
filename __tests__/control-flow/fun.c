#include "runtime_aly.h"

aly_value_t add(aly_value_t a, aly_value_t b);
aly_value_t stats(aly_value_t val1, aly_value_t val2);
aly_value_t ignore_first(aly_value_t _, aly_value_t val);
int main(int argc, char** argv) {
    aly_init(argc, argv);
    aly_value_t count = 0;
    aly_value_t x = 10;
    do {
    } while (aly_none());
    print;
    aly_value_t count2 = 0;
    aly_value_t y = 0;
    do {
    } while (aly_none());
    print;
    aly_value_t result = add;
    print;
    aly_value_t sum_val = aly_none();
    print;
    sum_val = 0;
    diff_val = 0;
    sum_val;
    print;
    _ = 100;
    aly_value_t _ = 200;
    aly_value_t sum_val2 = aly_none();
    print;
    sum_val2 = 0;
    sum_val2;
    print;
    aly_value_t ignore_res = ignore_first;
    print;
    aly_cleanup();
    return 0;
}

aly_value_t add(aly_value_t a, aly_value_t b) {
    aly_ref(a);
    aly_ref(b);
    return aly_add(a, b);
    return aly_none();
}
aly_value_t stats(aly_value_t val1, aly_value_t val2) {
    aly_ref(val1);
    aly_ref(val2);
    aly_value_t sum = aly_add(val1, val2);
    return aly_none();
}
aly_value_t ignore_first(aly_value_t _, aly_value_t val) {
    aly_ref(_);
    aly_ref(val);
    return val;
    return aly_none();
}
