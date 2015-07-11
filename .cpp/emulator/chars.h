#ifndef CHARS_H
#define CHARS_H

#include <cstdint>
#include <type_traits>
using namespace std;

struct Color { 
    uint8_t r, g, b;

    inline uint64_t hash() const {
        return (r << 16) + (g << 8) + b;
    }

    inline bool operator<(Color const& other) const { 
        return this->hash() < other.hash();
    }

    inline bool operator!=(Color const& other) const {
        return r != other.r || g != other.g || b != other.b;
    }
};
static_assert(is_pod<Color>::value, "Color must be a POD");


struct Attributes {
    bool standout, underline, reverse, blink, bold, dim, invisible, protected_, acs, italic;
    Color bg_color, fg_color;

    inline uint64_t hash() const {
        uint64_t v = (fg_color.hash() << 24) + bg_color.hash();
        v <<= 1; v |= standout;
        v <<= 1; v |= underline;
        v <<= 1; v |= reverse;
        v <<= 1; v |= blink;
        v <<= 1; v |= bold;
        v <<= 1; v |= dim;
        v <<= 1; v |= invisible;
        v <<= 1; v |= protected_;
        v <<= 1; v |= acs;
        v <<= 1; v |= italic;
        return v;
    }
};
static_assert(is_pod<Attributes>::value, "Attributes must be a POD");


struct Cell {
    char c[4];
    Attributes attr;

    inline uint64_t hash() const {
        return c[0] + (c[1] << 8) + (c[2] << 16) + (c[3] << 24) + (attr.hash() << 32);
    }

    inline bool operator<(Cell const& other) const {
        return this->hash() < other.hash();
    }
};
static_assert(is_pod<Cell>::value, "Cell must be a POD");


#endif

// vim: ts=4:sw=4:sts=4:expandtab
