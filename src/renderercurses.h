// Copyright 2015 André Wagner

#ifndef RENDERERCURSES_H_
#define RENDERERCURSES_H_

#include "renderer.h"

using namespace std;

class RendererCurses : public Renderer {
public:
    RendererCurses(Config const& config);
    virtual ~RendererCurses();

    void SetChar(int x, int y, char ch) override;
    void Refresh() override;

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

#endif  // RENDERERCURSES_H_

// vim: ts=4:sw=4:sts=4:expandtab
