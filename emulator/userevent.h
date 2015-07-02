#ifndef USEREVENT_H
#define USEREVENT_H

struct UserEvent {
    enum { NOTHING, KEYPRESS } type;
    union {
        uint8_t chr[4];
    };
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
