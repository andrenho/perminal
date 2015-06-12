// Copyright 2015 André Wagner

#include "client.h"


namespace client {


Client::Client(Config const& config, string const& hostname, string const& backend)
    : config(config)
{
    (void) hostname;
    (void) backend;
}


vector<uint8_t> 
Client::Read() const
{
    return { 0x2, 0x0, 0x0, 0x0, 0x0, 'A', 'n', 'd', 'r', 'e', 0 };
}


}  // namespace client

// vim: ts=4:sw=4:sts=4:expandtab
