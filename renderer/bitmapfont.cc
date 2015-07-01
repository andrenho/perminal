#include "bitmapfont.h"

#include <cassert>

#include "debug.h"

BitmapFont::BitmapFont(int char_width, int char_height, int image_width, int image_height)
    : Font(char_width, char_height, image_width, image_height)
{
}


BitmapFont 
BitmapFont::FromXBM(int w, int h, char* data)
{
    D("Loading XBM image...");

    int c = 0;

    BitmapFont f(w/16, h/16, w, h);
    f.data.reserve(w * h);
    for(int y=0; y<h; ++y) {
        for(int k=0; k<(w/8); ++k) {
            uint8_t px = static_cast<uint8_t>(data[(y*(w/8))+k]);
            for(int i=0; i<8; ++i) {
                f.data.push_back(((px >> i) & 1) ? 0 : 255);
                ++c;
            }
        }
    }
    ASSERT(c == (w*h), "c was expected to be %d, but is %d", (w*h), c);

    D("XBM image loaded.");

    return f;
}


// vim: ts=4:sw=4:sts=4:expandtab
