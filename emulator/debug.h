#ifndef DEBUG_H
#define DEBUG_H

#include <string>
using namespace std;

class Debug {
public:
    //void Info(string const& s) { Info("%s", s.c_str()); }
    void Info(const char* fmt, ...) __attribute__ ((format (printf, 2, 3)));
};

extern Debug debug;

#ifdef DEBUG
#  define D(...) debug.Info(__VA_ARGS__)
#else
#  define D(...)
#endif

#endif

// vim: ts=4:sw=4:sts=4:expandtab
