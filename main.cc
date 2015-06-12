#include <cstdlib>
#include <getopt.h>

#include "config.h"
#include "server/consoleplugin.h"
#include "server/servercharmatrix.h"
#include "server/server.h"
#include "client/client.h"
#include "client/clientcharmatrix.h"
#include "client/renderercurses.h"

int main(int argc, char** argv)
{
    Config config(argc, argv);

    if(config.InitializationType == Config::SERVER) {
        server::ConsolePlugin plugin(config, "./hello.so");
        server::ServerCharMatrix matrix(config, plugin);
        server::Server server(config, matrix);
        server.Serve();
    } else if(config.InitializationType == Config::CLIENT) {
        client::Client client(config, "localhost", "hello");
        client::ClientCharMatrix matrix(config, client);
        client::RendererCurses renderer(config, matrix);
        renderer.Execute();
    } else if(config.InitializationType == Config::DUAL) {
        // TODO
    } else {
        abort();
    }

    return EXIT_SUCCESS;
}

// vim: ts=4:sw=4:sts=4:expandtab
