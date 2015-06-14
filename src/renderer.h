// Copyright 2015 André Wagner

#ifndef RENDERER_H_
#define RENDERER_H_

using namespace std;

class Config;
class ConsolePlugin;
class Terminal;

class Renderer {
public:
    virtual ~Renderer() {}

    void SendInputToPlugin(ConsolePlugin const& plugin) const {}  // TODO
    void UpdateFromTerminal(Terminal const& terminal) {}  // TODO
    void Update() const {}  // TODO

protected:
    Renderer(Config const& config)
        : config(config) {}

    virtual void SetChar(int x, int y, char ch) = 0;
    virtual void Refresh() = 0;

    Config const& config;

private:
    Renderer(Renderer const&) = delete;
    Renderer(Renderer&&) = delete;
    Renderer& operator=(Renderer const&) = delete;
    Renderer& operator=(Renderer&&) = delete;

};

/*@
class Renderer {
    +Renderer()
}
@*/

#endif  // RENDERER_H_

// vim: ts=4:sw=4:sts=4:expandtab
