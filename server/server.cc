// Copyright 2015 André Wagner

#include "server.h"


namespace server {


Server::Server(Config const& config, ServerCharMatrix const& matrix)
    : config(config), matrix(matrix)
{
}


void
Server::Serve()
{
}


}  // namespace server

// vim: ts=4:sw=4:sts=4:expandtab
