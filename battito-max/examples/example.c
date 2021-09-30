#include <stdint.h>
#include <stdio.h>
#include "../src/battito_max.h"

int main (void) {
    char* a;
    char input[] = "1?10 2?20 3?30 4?40";
    struct event* buf;
    buf = transform(input, 480);
    printf("%d", buf[120].value);
}