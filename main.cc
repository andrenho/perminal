#include <cstdint>
#include <cstdlib>

#include <chrono>
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

#include "latin1.xbm"  // default font

Config config;
#ifdef DEBUG
Debug debug;
#endif

int main(int argc, char** argv)
{
    config.Initialize(argc, argv);

    try {

        const PTY plugin;
        Matrix matrix(80, 25);
        const Terminal terminal(matrix);

        const BitmapFont font = BitmapFont::FromXBM(latin1_width, latin1_height, latin1_bits, "ISO_8859-1");
        const XcbRenderer renderer(matrix, font);

        // get user input
        thread t_output([&terminal, &renderer, &plugin] {
            while(terminal.Alive() && renderer.Running()) {
                vector<uint8_t> data = terminal.ParseEvent(renderer.GetEvent());
                plugin.Write(data);
            }
        });

        // blink
        thread t_blink([&terminal, &renderer, &matrix] {
            while(terminal.Alive() && renderer.Running()) {
                matrix.Blink();
                this_thread::sleep_for(chrono::milliseconds(config.BlinkSpeed));
            }
        });

        // output to user
        while(terminal.Alive() && renderer.Running()) {
            vector<uint8_t> data = plugin.Read();
            terminal.ParseData(data);
            renderer.Update();
        }

        t_output.join();
        t_blink.join();

    } catch(RendererInitException& e) {
        fprintf(stderr, "perminal: %s\n", e.what());
        exit(EXIT_FAILURE);
    }

    return 0;
}


// vim: ts=4:sw=4:sts=4:expandtab
