// Copyright 2015 André Wagner

#ifndef SERVER_SERVER_H_
#define SERVER_SERVER_H_

using namespace std;

class Config;

namespace server {

class ServerCharMatrix;

class Server {
public:
    Server(Config const& config, ServerCharMatrix const& matrix); 
    virtual ~Server() {}

    void Serve();

private:
    Config const& config;
    ServerCharMatrix const& matrix;
    
    Server(Server const&) = delete;
    Server(Server&&) = delete;
    Server& operator=(Server const&) = delete;
    Server& operator=(Server&&) = delete;

};

/*@
class Server {
    +Server()
}
@*/

}  // namespace server

#endif  // SERVER_SERVER_H_

// vim: ts=4:sw=4:sts=4:expandtab
