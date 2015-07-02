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
    virtual CharImage LoadChar(char32_t c, Attributes const& attr) const = 0;

    int CharWidth() const { return char_width; }
    int CharHeight() const { return char_height; }

protected:
    Font(int char_width, int char_height, int image_width, int image_height)
        : char_width(char_width), char_height(char_height), image_width(image_width), image_height(image_height) {}

    int char_width;
    int char_height;
    int image_width;
    int image_height;
    vector<uint8_t> data = {};
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
