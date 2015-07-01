#include "debug.h"

#include <cstdio>
#include <cstdarg>

void 
Debug::Info(const char* fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    va_end(ap);
}

// vim: ts=4:sw=4:sts=4:expandtab
