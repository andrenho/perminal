#ifndef CHARS_H
#define CHARS_H

#include <cstdint>

struct Color { 
    uint8_t r, g, b;
    inline bool operator<(Color const& other) const { 
        uint32_t self = (r << 16) + (g << 8) + b,
                 oth  = (other.r << 16) + (other.g << 8) + other.b;
        return self < oth;
    }
};
static_assert(std::is_pod<Color>::value, "Color must be a POD");


struct Attributes {};  // TODO
static_assert(std::is_pod<Attributes>::value, "Attributes must be a POD");


struct Cell {
    char32_t c;
    Attributes attr;
};
static_assert(std::is_pod<Cell>::value, "Cell must be a POD");


#endif

// vim: ts=4:sw=4:sts=4:expandtab
