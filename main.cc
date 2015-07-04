#include <cstdint>
#include <cstdlib>

#include <chrono>
#include <memory>
#include <string>
#include <thread>
#include <vector>
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

        const PTY pty("xterm-256color");
        Matrix matrix(80, 25);
        const Terminal terminal(matrix);

        const BitmapFont font = BitmapFont::FromXBM(latin1_width, latin1_height, latin1_bits, "ISO_8859-1");
        const XcbRenderer renderer(matrix, font);

        while(terminal.Alive() && renderer.Running()) {
            // get user input
            vector<uint8_t> data_in = terminal.ParseEvent(renderer.GetEvent());
            pty.Write(data_in);

            // output things in the screen
            vector<uint8_t> data_out = pty.Read();
            terminal.ParseData(data_out);
            matrix.Update();

            // update renderer
            renderer.Update();
            
            // sleep
            this_thread::sleep_for(chrono::milliseconds(config.RenderUpdateMilliseconds));
        }

    } catch(RendererInitException& e) {
        fprintf(stderr, "perminal: %s\n", e.what());
        exit(EXIT_FAILURE);
    }

    return 0;
}


// vim: ts=4:sw=4:sts=4:expandtab
