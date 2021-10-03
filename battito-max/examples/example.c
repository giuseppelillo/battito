#include <stdint.h>
#include <stdio.h>
#include "../src/battito_max.h"

int main (void) {
    char input[] = "1 2 3";
    struct event* buf;
    struct pattern pattern = transform(input, 480);
    printf("%d\n", pattern.events[120].value);
    printf("%d\n", pattern.length);
}