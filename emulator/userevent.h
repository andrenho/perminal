#ifndef USEREVENT_H
#define USEREVENT_H

enum UserEventType { NOTHING, KEYPRESS }; 

struct UserEvent {
    UserEventType type;
    union {
        uint8_t chr[5] = { 0, 0, 0, 0, 0 };
    };

    UserEvent(UserEventType type) : type(type) {}
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
