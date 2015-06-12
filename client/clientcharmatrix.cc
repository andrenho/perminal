// Copyright 2015 André Wagner

#include "clientcharmatrix.h"

#include <algorithm>
#include <sstream>

#include "command.h"
#include "client.h"

namespace client {


ClientCharMatrix::ClientCharMatrix(Config const& config, Client const& client)
    : config(config), client(client)
{
}


vector<CharUpdate> const& 
ClientCharMatrix::Updates()
{
    Update();
    return updates;
}


void 
ClientCharMatrix::Update()
{
    // read data from client
    auto bytes = client.Read();
    if(!bytes.empty()) {
        commands.insert(commands.end(), bytes.begin(), bytes.end());
        
        // execute commands
        while(!commands.empty()) {
            uint16_t x = 0, y = 0;
            uint8_t ch = 0;
            stringstream ss;
            unsigned int i = 0;

            switch(commands.front()) {
                case Command::WRITE_CHAR:
                    if(commands.size() < 5) {
                        return;  // incomplete data
                    }
                    x = (commands[1] << 8) + commands[2];
                    y = (commands[3] << 8) + commands[4];
                    ch = commands[5];
                    updates.push_back({ ch, x, y });
                    commands.erase(commands.begin(), commands.begin()+6);
                    break;
                case Command::WRITE_STREAM:
                    if(commands.size() < 4) {
                        return;
                    }
                    x = (commands[1] << 8) + commands[2];
                    y = (commands[3] << 8) + commands[4];
                    i = 5;
                    while(true) {
                        if(commands.size() < i) {
                            return;
                        }
                        ch = commands[i++];
                        if(ch == 0) {
                            break;
                        }
                        updates.push_back({ ch, x, y });
                    };
                    commands.erase(commands.begin(), commands.begin()+i);
                    break;
            }
        }
    }

}


}  // namespace client

// vim: ts=4:sw=4:sts=4:expandtab
