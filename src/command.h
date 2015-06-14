// Copyright 2015 André Wagner

#include <cstdint>

enum Command : uint8_t {
    NOOP                    = 0x0,
    WRITE_CHAR              = 0x1,
    WRITE_STREAM            = 0x2,
};

// vim: ts=4:sw=4:sts=4:expandtab
