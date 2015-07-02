#include <cstdint>
#include <cstdlib>
#include <memory>
#include <string>
#include <vector>
#include <thread>
using namespace std;

#include "config.h"
#include "plugin.h"
#include "terminal.h"
#include "font.h"
#include "renderer.h"
#include "userevent.h"
#include "debug.h"

#include "pty.h"
#include "bitmapfont.h"
#include "xcbrenderer.h"

#include "Vintl01.xbm"

Config config;
#ifdef DEBUG
Debug debug;
#endif

int main(int argc, char** argv)
{
    config.Initialize(argc, argv);

    try {

        const PTY plugin;
        const Matrix matrix(80, 25);
        const Terminal terminal(matrix);

        const BitmapFont font = BitmapFont::FromXBM(Vintl01_width, Vintl01_height, Vintl01_bits, "ISO_8859-1");
        const XcbRenderer renderer(matrix, font);

        // get user input
        thread t_output([&terminal, &renderer, &plugin] {
            while(terminal.Alive() && renderer.Running()) {
                vector<uint8_t> data = terminal.ParseEvent(renderer.GetEvent());
                plugin.Write(data);
            }
        });

        // output to user
        while(terminal.Alive() && renderer.Running()) {
            vector<uint8_t> data = plugin.Read();
            terminal.ParseData(data);
            renderer.Update();
        }

        t_output.join();

    } catch(RendererInitException& e) {
        fprintf(stderr, "perminal: %s\n", e.what());
        exit(EXIT_FAILURE);
    }

    return 0;
}


// vim: ts=4:sw=4:sts=4:expandtab
