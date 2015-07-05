#ifndef USEREVENT_H
#define USEREVENT_H

enum UserEventType { NOTHING, RESIZE, KEYPRESS }; 

struct UserEvent {
    UserEventType type;
    union {
        uint8_t chr[5] = { 0, 0, 0, 0, 0 };
        uint16_t size[2];
    };

    explicit UserEvent(UserEventType type) : type(type) {}
    UserEvent(UserEventType type, uint8_t c[5]) : type(type) {
        chr[0] = c[0];
        chr[1] = c[1];
        chr[2] = c[2];
        chr[3] = c[3];
        chr[4] = c[4];
    }
    UserEvent(UserEventType type, char c[5]) : type(type) {
        chr[0] = c[0];
        chr[1] = c[1];
        chr[2] = c[2];
        chr[3] = c[3];
        chr[4] = c[4];
    }
    UserEvent(UserEventType type, uint16_t w, uint16_t h) : type(type) {
        size[0] = w;
        size[1] = h;
    }
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
