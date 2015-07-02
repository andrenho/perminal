#include "bitmapfont.h"

#include <cassert>

#include "config.h"
#include "debug.h"

BitmapFont::BitmapFont(int char_width, int char_height, int image_width, int image_height)
    : Font(char_width, char_height, image_width, image_height)
{
}


BitmapFont 
BitmapFont::FromXBM(int w, int h, unsigned char* data)
{
    D("Loading XBM image...");

    int c = 0;

    BitmapFont f(w/16, h/16, w, h);
    f.data.reserve(w * h);
    uint8_t bg = data[0] & 1;
    for(int y=0; y<h; ++y) {
        for(int k=0; k<(w/8); ++k) {
            uint8_t px = data[(y*(w/8))+k];
            for(int i=0; i<8; ++i) {  // TODO - check if is divisible by 8
                f.data.push_back(((px >> i) & 1) == bg ? 0 : 255);
                ++c;
            }
        }
    }
    ASSERT(c == (w*h), "c was expected to be %d, but is %d", (w*h), c);

    D("XBM image loaded.");

    return f;
}


CharImage 
BitmapFont::LoadChar(const char chr[4], Attributes const& attr) const
{
    uint8_t c = (chr[1] > 0) ? config.Invalid8bitChar : chr[0];   // TODO
    int x_in_image = (c % 16) * char_width,
        y_in_image = (c / 16) * char_height;

    // create base image
    vector<uint8_t> px_image;
    px_image.reserve(char_width * char_height);
    for(int y = y_in_image; y<(y_in_image + char_height); ++y) {
        for(int x = x_in_image; x<(x_in_image + char_width); ++x) {
            uint8_t px = this->data[(y*image_width)+x];
            px_image.push_back(px);
        }
    }

    // apply attributes
    if(attr.italic) {
        ApplyItalic(px_image);
    }
    if(attr.bold) {
        ApplyBold(px_image);
    }
    if(attr.underline) {
        ApplyUnderline(px_image);
    }
    if(attr.dim) {
        ApplyDim(px_image);
    }
    if(attr.standout || attr.reverse) {
        ApplyReverse(px_image);
    }
    if(attr.invisible) {
        ApplyInvisible(px_image);
    }

    // apply base image
    CharImage ch { char_width, char_height, {}, attr.bg_color };
    ch.data.reserve(char_width * char_height);
    for(auto const& px: px_image) {
        double ppx = static_cast<double>(px) / 255.0;
        ch.data.push_back({
            attr.bg_color.r + (static_cast<double>(attr.fg_color.r-attr.bg_color.r) * ppx),
            attr.bg_color.g + (static_cast<double>(attr.fg_color.g-attr.bg_color.g) * ppx),
            attr.bg_color.b + (static_cast<double>(attr.fg_color.b-attr.bg_color.b) * ppx)
        });
    }

    ASSERT(static_cast<int>(ch.data.size()) == (char_width*char_height), 
            "ch.data.size was expected to be %d, but is %zu", (char_width*char_height), ch.data.size());

    return ch;
}


void
BitmapFont::ApplyItalic(vector<uint8_t>& px_image) const
{
    for(int y=0; y<char_height/3; ++y) {
        for(int x=char_width-1; x>0; --x) {
            const int p = x + (y * char_width);
            px_image[p] = px_image[p-1];
        }
    }
    for(int y=char_height/3*2-1; y<char_height; ++y) {
        for(int x=0; x<char_width-1; ++x) {
            const int p = x + (y * char_width);
            px_image[p] = px_image[p+1];
        }
    }
}


void
BitmapFont::ApplyDim(vector<uint8_t>& px_image) const
{
    for(auto& px: px_image) {
        px *= config.DimPercentage;
    }
}


void
BitmapFont::ApplyReverse(vector<uint8_t>& px_image) const
{
    for(auto& px: px_image) {
        px = ~px;
    }
}


void
BitmapFont::ApplyUnderline(vector<uint8_t>& px_image) const
{
    for(int x=0; x<char_width; ++x) {
        const int p = x+((char_height - config.UnderlineY) * char_width);
        px_image[p] = config.UnderlineIntensity;
    }
}


void
BitmapFont::ApplyBold(vector<uint8_t>& px_image) const
{
    for(int y=0; y<char_height; ++y) {
        for(int x=char_width-1; x>0; --x) {
            const int p = x + (y * char_width);
            if(px_image[p-1] > 64) {  // TODO ???
                px_image[p] = px_image[p-1];
            }
        }
    }
}


void
BitmapFont::ApplyInvisible(vector<uint8_t>& px_image) const
{
    for(auto& px: px_image) {
        px = 0;
    }
}


// vim: ts=4:sw=4:sts=4:expandtab
