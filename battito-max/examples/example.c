#include <stdint.h>
#include <stdio.h>
#include "../src/battito_max.h"

int main (void) {
    char* a;
    char input[] = " ";
    struct event* buf;
    struct pattern pattern = transform(input, 480);
    printf("%d", pattern.events[120].value);
    printf("%d", pattern.length);
}