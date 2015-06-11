#include <cstdlib>
#include <getopt.h>

int main(int argc, char** argv)
{
    if(argv == 1) {
        if(string(argv[0]) == "-s") {
            ConsolePlugin plugin("./hello.so");
            ServerCharMatrix matrix(plugin);
            Server server(matrix);
            server.Serve();
        } else if(string(argv[0]) == "-c") {
            Client client("localhost");
            ClientCharMatrix matrix(client);
            RendererCurses renderer(matrix);
            renderer.Execute();
        } else {
            fprintf("Invalid option %s\n", argv[0]);
            return EXIT_FAILURE;
        }
    }
    

    

    return EXIT_SUCCESS;
}

// vim: ts=4:sw=4:sts=4:expandtab
