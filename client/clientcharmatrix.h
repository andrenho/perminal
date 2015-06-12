// Copyright 2015 André Wagner

#ifndef CLIENT_CLIENTCHARMATRIX_H_
#define CLIENT_CLIENTCHARMATRIX_H_

using namespace std;

class Config;

namespace client {

class Client;

class ClientCharMatrix {
public:
    ClientCharMatrix(Config const& config, Client const& client); 
    virtual ~ClientCharMatrix() {}
private:
    Config const& config;
    Client const& client;

    ClientCharMatrix(ClientCharMatrix const&) = delete;
    ClientCharMatrix(ClientCharMatrix&&) = delete;
    ClientCharMatrix& operator=(ClientCharMatrix const&) = delete;
    ClientCharMatrix& operator=(ClientCharMatrix&&) = delete;

};

/*@
class ClientCharMatrix {
    +ClientCharMatrix()
}
@*/

}  // namespace client

#endif  // CLIENT_CLIENTCHARMATRIX_H_

// vim: ts=4:sw=4:sts=4:expandtab
