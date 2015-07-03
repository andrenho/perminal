#ifndef CHARENCODING_H
#define CHARENCODING_H

#include <iconv.h>

#include <string>
using namespace std;

class CharEncoding {
public:
    CharEncoding(string const& from, string const& to);
    ~CharEncoding();

    bool Convert(const char from[4], char to[4]) const;

    // uncopyable
    CharEncoding(CharEncoding const& ce) = default;
    CharEncoding(CharEncoding&& ce) = default;
    CharEncoding& operator=(CharEncoding const& ce) = default;
    CharEncoding& operator=(CharEncoding&& ce) = default;
private:
    iconv_t cd;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
