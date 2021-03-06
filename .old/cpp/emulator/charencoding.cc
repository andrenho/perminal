#include "charencoding.h"

#include <cstring>

#include <stdexcept>
using namespace std;

#include "config.h"
#include "debug.h"

CharEncoding::CharEncoding(string const& from, string const& to)
    : cd(iconv_open(to.c_str(), from.c_str()))
{
    if(cd == reinterpret_cast<iconv_t>(-1)) {
        throw runtime_error("Conversion from " + from + " to " + to + " is not avaliable");
    }
}


CharEncoding::~CharEncoding()
{
    iconv_close(cd);
}


bool
CharEncoding::Convert(const char from[4], char to[4]) const
{
    size_t inbytesleft = strlen(from);
    size_t outbytesleft = 4;

    if(iconv(cd, const_cast<char**>(&from), &inbytesleft, &to, &outbytesleft) == static_cast<size_t>(-1)) {
        switch(errno) {
        case E2BIG:
            throw logic_error("Invalid outbuf size in iconv.");
        case EILSEQ:
        case EINVAL:
            to[0] = config.Invalid8bitChar;
            to[1] = 0;
            return false;
        }
    }

    return true;
}


bool 
CharEncoding::IsComplete(const char c[4], int sz) const
{
    // TODO OPTIMIZE - iconv is not necessary for this, we can do it by hand

    if(static_cast<uint8_t>(c[0]) < 128) {
        return true;
    }

    size_t inbytesleft = sz; //strlen(c);
    size_t outbytesleft = 4;
    char a[4] = { 0, 0, 0, 0 };
    char* to = a;

    if(iconv(cd, const_cast<char**>(&c), &inbytesleft, &to, &outbytesleft) == static_cast<size_t>(-1)) {
        switch(errno) {
        case E2BIG:
            throw logic_error("Invalid outbuf size in iconv.");
        case EILSEQ:
            D("Invalid multibyte sequence 0x%02x 0x%02x 0x%02x 0x%02x", c[0], c[1], c[2], c[3]);
            break;
        case EINVAL:
            return false;
        }
    }

    return true;
}

// vim: ts=4:sw=4:sts=4:expandtab
