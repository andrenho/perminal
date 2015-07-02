#ifndef BITMAPFONT_H
#define BITMAPFONT_H

#include <cstdint>
#include <string>
#include <vector>
using namespace std;

#include "font.h"

class BitmapFont : public Font {
public:
    static BitmapFont FromXBM(int w, int h, unsigned char* data);
    CharImage LoadChar(const char c[4], Attributes const& attr) const;

private:
    BitmapFont(int char_width, int char_height, int image_width, int image_height);
    
    void ApplyItalic(vector<uint8_t>& px_image) const;
    void ApplyUnderline(vector<uint8_t>& px_image) const;
    void ApplyReverse(vector<uint8_t>& px_image) const;
    void ApplyBold(vector<uint8_t>& px_image) const;
    void ApplyDim(vector<uint8_t>& px_image) const;
    void ApplyInvisible(vector<uint8_t>& px_image) const;
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
