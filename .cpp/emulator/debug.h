#ifndef DEBUG_H
#define DEBUG_H

#ifdef DEBUG

#include <functional>
#include <string>
using namespace std;

class Debug {
public:
    //void Info(string const& s) { Info("%s", s.c_str()); }
    void Info(const char* fmt, ...) const __attribute__ ((format (printf, 2, 3)));

    void InfoCharacter(char c, bool complete=true, bool cap=false) const;

    void Assert(bool validation) const;
    void Assert(function<bool()> v_func) const;
    void Assert(bool validation, const char* fmt, ...) const __attribute__ ((format (printf, 3, 4)));
    void Assert(function<bool()> v_func, const char* fmt, ...) const __attribute__ ((format (printf, 3, 4)));
};

extern Debug debug;

#define C(...) debug.InfoCharacter(__VA_ARGS__)
#define D(...) debug.Info(__VA_ARGS__)
#define ASSERT(...) debug.Assert(__VA_ARGS__)

#else
#  define C(...)
#  define D(...)
#  define ASSERT(...)
#endif

#endif

// vim: ts=4:sw=4:sts=4:expandtab
