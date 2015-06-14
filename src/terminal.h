// Copyright 2015 André Wagner

#ifndef TERMINAL_H_
#define TERMINAL_H_

using namespace std;

class Config;

class Terminal {
public:
    Terminal(Config const& config); 
    virtual ~Terminal() {}

private:
    Config const& config;

    Terminal(Terminal const&) = delete;
    Terminal(Terminal&&) = delete;
    Terminal& operator=(Terminal const&) = delete;
    Terminal& operator=(Terminal&&) = delete;

};

/*@
class Terminal {
    +Terminal()
}
@*/

#endif  // SRC_TERMINAL_H_

// vim: ts=4:sw=4:sts=4:expandtab
