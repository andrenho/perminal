#include <cstdlib>
#include <getopt.h>

#include "config.h"
#include "consoleplugin.h"
#include "terminal.h"
#include "renderercurses.h"

int main(int argc, char** argv)
{
    Config config(argc, argv);

    ConsolePlugin plugin(config, "./hello.so");
    Terminal terminal(config);
    RendererCurses renderer(config);

    while(true) {
        renderer.SendInputToPlugin(plugin);
        
        plugin.SendTextToTerminal(terminal);
        renderer.UpdateFromTerminal(terminal);

        renderer.Update();
    }

    return EXIT_SUCCESS;
}

// vim: ts=4:sw=4:sts=4:expandtab
