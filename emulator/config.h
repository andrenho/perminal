#ifndef CONFIG_H
#define CONFIG_H

#include <cstdint>

#include "chars.h"

struct TBorderSize {
    uint16_t LeftRight;
    uint16_t TopBottom;
};

class Config {
public:
    void Initialize(int argc, char** argv) { (void) argc; (void) argv; }

    // default colors
    Color  DefaultBGColor = { 255, 255, 255 };
    Color  DefaultFGColor = { 0, 0, 0 };

    // border
    Color       BorderColor = { 128, 255, 128 };
    TBorderSize BorderSize  = { 30, 30 };

    // font
    double DimPercentage = 0.5;
    int UnderlineY = 2;
    int UnderlineIntensity = 255;

    // encoding
    uint8_t Invalid8bitChar = '?';
};

extern Config config;

#endif

// vim: ts=4:sw=4:sts=4:expandtab
