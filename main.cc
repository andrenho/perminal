#include <cstdint>
#include <cstdlib>
#include <memory>
#include <string>
#include <vector>
#include <thread>
using namespace std;

#include "plugin.h"
#include "terminal.h"
#include "font.h"
#include "renderer.h"
#include "userevent.h"

#include "pty.h"
#include "bitmapfont.h"
#include "xcbrenderer.h"

int main(int argc, char** argv)
{
    (void) argc;
    (void) argv;

    try {

        const PTY plugin;
        const Terminal terminal;

        const BitmapFont font("Sleroux_800x300.bmp");
        const XcbRenderer renderer(font);

        // get user input
        thread t_output([&terminal, &renderer, &plugin] {
            while(terminal.Alive() && renderer.Running()) {
                vector<UserEvent> events = renderer.GetEvents();
                for(auto const& event: events) {
                    vector<uint8_t> data = terminal.ParseEvent(event);
                    plugin.Write(data);
                }
            }
        });

        // output to user
        while(terminal.Alive() && renderer.Running()) {
            vector<uint8_t> data = plugin.Read();
            auto const& matrix = terminal.ParseData(data);
            renderer.Update(matrix);
        }

        t_output.join();

    } catch(RendererInitException& e) {
        fprintf(stderr, "perminal: %s\n", e.what());
        exit(EXIT_FAILURE);
    }

    return 0;
}


// vim: ts=4:sw=4:sts=4:expandtab
