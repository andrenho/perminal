// Copyright 2015 André Wagner

#ifndef SERVER_SERVERCHARMATRIX_H_
#define SERVER_SERVERCHARMATRIX_H_

using namespace std;

class Config;

namespace server {

class ConsolePlugin;

class ServerCharMatrix {
public:
    ServerCharMatrix(Config const& config, ConsolePlugin const& plugin); 
    virtual ~ServerCharMatrix() {} 
private:
    Config const& config;
    ConsolePlugin const& plugin;

    ServerCharMatrix(ServerCharMatrix const&) = delete;
    ServerCharMatrix(ServerCharMatrix&&) = delete;
    ServerCharMatrix& operator=(ServerCharMatrix const&) = delete;
    ServerCharMatrix& operator=(ServerCharMatrix&&) = delete;

};

/*@
class ServerCharMatrix {
    +ServerCharMatrix()
}
@*/

}  // namespace server

#endif  // SERVER_SERVERCHARMATRIX_H_

// vim: ts=4:sw=4:sts=4:expandtab
