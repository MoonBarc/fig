#include <stdio.h>

extern int fig_entrypoint();

int main() {
    printf("running code!\n");
    // TODO: this is here for debugging purposes.
    // it would be cool if this was all just asm
    printf("result = %i\n", fig_entrypoint());
    return 0;
}
