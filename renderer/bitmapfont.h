#ifndef BITMAPFONT_H
#define BITMAPFONT_H

#include <cstdint>
#include <string>
#include <vector>
using namespace std;

#include "font.h"

class BitmapFont : public Font {
public:
    static BitmapFont FromXBM(int w, int h, char* data);

private:
    BitmapFont(int char_width, int char_height, int image_width, int image_height);
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
