// Copyright 2015 André Wagner

#ifndef CLIENT_CLIENT_H_
#define CLIENT_CLIENT_H_

#include <string>
using namespace std;

class Config;

namespace client {

class Client {
public:
    Client(Config const& config, string const& hostname, string const& backend); 
    virtual ~Client() {}
private:
    Config const& config;

    Client(Client const&) = delete;
    Client(Client&&) = delete;
    Client& operator=(Client const&) = delete;
    Client& operator=(Client&&) = delete;

};

/*@
class Client {
    +Client()
}
@*/

}  // namespace client

#endif  // CLIENT_CLIENT_H_

// vim: ts=4:sw=4:sts=4:expandtab
