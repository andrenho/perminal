#ifndef BITMAPFONT_H
#define BITMAPFONT_H

#include <string>
using namespace std;

#include "font.h"

class BitmapFont : public Font {
public:
    explicit BitmapFont(string const& filename) { (void) filename; }
    virtual CharImage LoadChar(char32_t c, Attributes const& attr) { (void)c; (void)attr; return {}; }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
