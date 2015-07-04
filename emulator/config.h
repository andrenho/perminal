#ifndef CONFIG_H
#define CONFIG_H

#include <cstdint>

#include "chars.h"

struct TBorderSize {
    uint16_t LeftRight;
    uint16_t TopBottom;
};

enum ECursorType { BACKGROUND, ALTERNATE_CHAR, OVERWRITE_CHAR };

class Config {
public:
    void Initialize(int argc, char** argv) { (void) argc; (void) argv; }

    // console
    Color  DefaultBGColor = { 255, 255, 255 };
    Color  DefaultFGColor = { 0, 0, 0 };
    int    BlinkSpeed = 400;

    // cursor
    bool        BlinkCursor = true;
    ECursorType CursorType = BACKGROUND;
    Color       CursorVisibleColor = { 0, 255, 0 };
    Color       CursorVeryVisibleColor = { 255, 0, 255 };

    // font
    double DimPercentage = 0.5;
    int UnderlineY = 2;
    int UnderlineIntensity = 255;

    // encoding
    uint8_t Invalid8bitChar = 255;

    // window
    Color       BorderColor = { 128, 255, 128 };
    TBorderSize BorderSize  = { 30, 30 };

    // usage
    int RenderUpdateMilliseconds = 20;
};

extern Config config;

#endif

// vim: ts=4:sw=4:sts=4:expandtab
