#include <stdio.h>

FILE* libc_stdhandle_rs_stdin() {
    return stdin;
}

FILE* libc_stdhandle_rs_stdout() {
    return stdout;
}

FILE* libc_stdhandle_rs_stderr() {
    return stderr;
}
