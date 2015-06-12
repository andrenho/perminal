// Copyright 2015 André Wagner

#ifndef CLIENT_CLIENTCHARMATRIX_H_
#define CLIENT_CLIENTCHARMATRIX_H_

#include <cstdint>
#include <vector>
#include <deque>
using namespace std;

class Config;

namespace client {

class Client;

struct CharUpdate {
    int ch;
    int x, y;
};

class ClientCharMatrix {
public:
    ClientCharMatrix(Config const& config, Client const& client); 
    virtual ~ClientCharMatrix() {}

    inline void ClearUpdates() { updates.clear(); }
    vector<CharUpdate> const& Updates();

private:
    void Update();

    Config const& config;
    Client const& client;

    vector<CharUpdate> updates = {};
    deque<uint8_t> commands = {};

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
