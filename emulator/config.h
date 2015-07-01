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

    Color       BorderColor = { 128, 255, 128 };
    TBorderSize BorderSize = { 30, 30 };
};

extern Config config;

#endif

// vim: ts=4:sw=4:sts=4:expandtab
