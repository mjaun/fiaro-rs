#include <stdio.h>
#include <stdlib.h>
#include "diag/Trace.h"

extern void rust_main(void);

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-parameter"

int main(int argc, char* argv[]) {
    trace_printf("Hello from C!\n");
    rust_main();
    return 0;
}

#pragma GCC diagnostic pop
