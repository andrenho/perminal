// Copyright 2015 André Wagner

#ifndef CLIENT_RENDERERCURSES_H_
#define CLIENT_RENDERERCURSES_H_

#include "renderer.h"

using namespace std;

namespace client {

class RendererCurses : public Renderer {
public:
    RendererCurses(Config const& config, ClientCharMatrix const& matrix);
    virtual ~RendererCurses() {}

    void Execute() override;
    
private:
    RendererCurses(RendererCurses const&) = delete;
    RendererCurses(RendererCurses&&) = delete;
    RendererCurses& operator=(RendererCurses const&) = delete;
    RendererCurses& operator=(RendererCurses&&) = delete;

};

/*@
class RendererCurses {
    +RendererCurses()
}
@*/

}  // namespace client

#endif  // CLIENT_RENDERERCURSES_H_

// vim: ts=4:sw=4:sts=4:expandtab
