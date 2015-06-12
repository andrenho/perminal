// Copyright 2015 André Wagner

#include "client.h"


namespace client {


Client::Client(Config const& config, string const& hostname, string const& backend)
    : config(config)
{
    (void) hostname;
    (void) backend;
}


}  // namespace client

// vim: ts=4:sw=4:sts=4:expandtab
