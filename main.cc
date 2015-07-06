#include <cstdint>
#include <cstdlib>

#include <chrono>
#include <memory>
#include <string>
#include <thread>
#include <vector>
using namespace std;

#include "config.h"
#include "commands.h"
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

        const Terminal terminal;

        const PTY pty(terminal.TERM());
        Matrix matrix(80, 25);

        const BitmapFont font = BitmapFont::FromXBM(latin1_width, latin1_height, latin1_bits, "ISO_8859-1");
        const XcbRenderer renderer(matrix, font);

        uint8_t* buffer = new uint8_t[config.BufferSize];
        uint32_t pars[256];
        uint32_t u = 0;
        while(renderer.Running()) {

            // get user input
            const UserEvent event = renderer.GetEvent();
            if(event.type == RESIZE) {
                pty.Resize(event.size[0], event.size[1]);
                matrix.Resize(event.size[0], event.size[1]);
            }
            if(event.type != NOTHING) {
                const int n = terminal.ParseUserEvent(event, buffer);
                if(n) {
                    pty.Write(buffer, n);
                }
            }

            // output things in the screen
            const int m = pty.Read(buffer, config.BufferSize);
            for(int i=0; i<m; ++i) {
                Command cmd = terminal.ParsePluginOutput(buffer[i], pars);
                matrix.Do(cmd, pars);
            }
            if(m == -1) {
                break;  // the connection was cut
            }

            if(m == 0) {
                // update renderer
                matrix.Update();
            }

            renderer.Update();
            
            // sleep
            if(m == 0) {  // sleep only if there was no input
                this_thread::sleep_for(chrono::milliseconds(config.RenderUpdateMilliseconds));
            }

            ++u;
        }
        delete[] buffer;

    } catch(RendererInitException& e) {
        fprintf(stderr, "perminal: %s\n", e.what());
        exit(EXIT_FAILURE);
    }

    return 0;
}


// vim: ts=4:sw=4:sts=4:expandtab
