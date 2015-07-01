#ifndef FONT_H
#define FONT_H

#include <vector>
using namespace std;

#include "chars.h"

struct CharImage {
    int w;
    int h;
    vector<Color> data;
    Color bg_color;
};


class Font {
public:
    virtual ~Font() {}
    virtual CharImage LoadChar(char32_t c, Attributes const& attr) = 0;

    int CharWidth() const { return char_width; }
    int CharHeight() const { return char_height; }

protected:
    int char_width = 0;
    int char_height = 0;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
