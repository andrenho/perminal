#include "font.h"

#include "debug.h"

CharImage 
Font::LoadChar(char32_t c, Attributes const& attr) const
{
    CharImage ch { char_width, char_height, {}, attr.bg_color };
    ch.data.reserve(char_width * char_height);
    int x_in_image = (c % 16) * char_width,
        y_in_image = (c / 16) * char_height;
    for(int y = y_in_image; y<(y_in_image + char_height); ++y) {
        for(int x = x_in_image; x<(x_in_image + char_width); ++x) {
            uint8_t px = this->data[(y*image_height)+x];
            double ppx = static_cast<double>(px) / 255.0;
            ch.data.push_back({
                attr.bg_color.r + (static_cast<double>(attr.fg_color.r-attr.bg_color.r) * ppx),
                attr.bg_color.g + (static_cast<double>(attr.fg_color.g-attr.bg_color.g) * ppx),
                attr.bg_color.b + (static_cast<double>(attr.fg_color.b-attr.bg_color.b) * ppx)
            });
        }
    }
    ASSERT(static_cast<int>(ch.data.size()) == (char_width*char_height), 
            "ch.data.size was expected to be %d, but is %d", (char_width*char_height), ch.data.size());
    return ch;
}


// vim: ts=4:sw=4:sts=4:expandtab
