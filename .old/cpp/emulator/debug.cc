#include "debug.h"

#include <cstdio>
#include <cstdlib>
#include <cstdarg>

#ifdef DEBUG

void 
Debug::Info(const char* fmt, ...) const
{
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    fprintf(stderr, "\n");
    va_end(ap);
}


void 
Debug::InfoCharacter(char c, bool complete, bool cap) const
{
    if(cap) { fprintf(stderr, "\e[7m"); }
    fprintf(stderr, "[%3d '%c'%c ", static_cast<unsigned char>(c), c >= 32 && c < 127 ? c : '#', complete ? ']' : '>');
    if(cap) { fprintf(stderr, "\e[27m"); }
}


void 
Debug::Assert(bool validation) const
{
    if(!validation) {
        abort();
    }
}


void 
Debug::Assert(function<bool()> v_func) const
{
    if(!v_func()) {
        abort();
    }
}


void 
Debug::Assert(bool validation, const char* fmt, ...) const
{
    if(!validation) {
        va_list ap;
        va_start(ap, fmt);
        fprintf(stderr, "Assertion failed: ");
        vfprintf(stderr, fmt, ap);
        fprintf(stderr, "\n");
        va_end(ap);
        abort();
    }
}


void 
Debug::Assert(function<bool()> v_func, const char* fmt, ...) const
{
    if(!v_func()) {
        va_list ap;
        va_start(ap, fmt);
        fprintf(stderr, "Assertion failed: ");
        vfprintf(stderr, fmt, ap);
        fprintf(stderr, "\n");
        va_end(ap);
        abort();
    }
}

#endif

// vim: ts=4:sw=4:sts=4:expandtab
