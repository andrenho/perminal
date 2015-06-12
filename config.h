// Copyright 2015 André Wagner

#ifndef CONFIG_H_
#define CONFIG_H_

using namespace std;

class Config {
public:
    Config(int argc, char** argv); 
    virtual ~Config() {}

    enum { SERVER, CLIENT, DUAL } InitializationType = CLIENT;

private:
    Config(Config const&) = delete;
    Config(Config&&) = delete;
    Config& operator=(Config const&) = delete;
    Config& operator=(Config&&) = delete;
};

/*@
class Config {
    +Config()
}
@*/

#endif  // CONFIG_H_

// vim: ts=4:sw=4:sts=4:expandtab
