#ifndef USEREVENT_H
#define USEREVENT_H

#include <cstdint>

enum UserEventType : uint8_t { NOTHING, RESIZE, KEYPRESS }; 

enum SpecialKey {
    NO_KEY,
    UP, DOWN, LEFT, RIGHT,
};

struct UserEvent {
    UserEventType type;
    union {
        uint8_t chr[5] = { 0, 0, 0, 0, 0 };
        uint16_t size[2];
        SpecialKey key;
    };

    explicit UserEvent(UserEventType type) : type(type) {}
    UserEvent(UserEventType type, uint8_t c[5]) : type(type), chr { c[0], c[1], c[2], c[3], c[4] } {}
    UserEvent(UserEventType type, char c[5]) : type(type), chr { c[0], c[1], c[2], c[3], c[4] } {}
    UserEvent(UserEventType type, uint16_t w, uint16_t h) : type(type), size { w, h } {}
    UserEvent(UserEventType type, SpecialKey key) : type(type), key(key) { }

    inline uint64_t hash() const {
        return (static_cast<uint64_t>(type) << 40) + 
            (static_cast<uint64_t>(chr[4]) << 32) + 
            (static_cast<uint64_t>(chr[3]) << 24) + 
            (static_cast<uint64_t>(chr[2]) << 16) + 
            (static_cast<uint64_t>(chr[1]) << 8) + 
            static_cast<uint64_t>(chr[0]);
    }

    inline bool operator<(UserEvent const& other) const { 
        return this->hash() < other.hash();
    }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
