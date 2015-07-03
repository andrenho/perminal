#include "cursor.h"

#include <cstdlib>

Color 
Cursor::color() const
{
    switch(intensity) {
        case VISIBLE:      return config.CursorVisibleColor;
        case VERY_VISIBLE: return config.CursorVeryVisibleColor;
        case INVISIBLE:
        default: abort();  // we should't be asking for the color
    }
    
}

// vim: ts=4:sw=4:sts=4:expandtab
