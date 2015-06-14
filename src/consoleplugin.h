// Copyright 2015 André Wagner

#ifndef CONSOLEPLUGIN_H_
#define CONSOLEPLUGIN_H_

#include <string>
using namespace std;

class Config;
class Terminal;

class ConsolePlugin {
public:
    ConsolePlugin(Config const& config, string const& plugin_file);
    virtual ~ConsolePlugin() {}

    void SendTextToTerminal(Terminal const& terminal) {}  // TODO

private:
    Config const& config;

    ConsolePlugin(ConsolePlugin const&) = delete;
    ConsolePlugin(ConsolePlugin&&) = delete;
    ConsolePlugin& operator=(ConsolePlugin const&) = delete;
    ConsolePlugin& operator=(ConsolePlugin&&) = delete;

};

/*@
class ConsolePlugin {
    +ConsolePlugin()
}
@*/

#endif  // SERVER_CONSOLEPLUGIN_H_

// vim: ts=4:sw=4:sts=4:expandtab
