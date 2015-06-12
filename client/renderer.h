// Copyright 2015 André Wagner

#ifndef CLIENT_RENDERER_H_
#define CLIENT_RENDERER_H_

using namespace std;

class Config;

namespace client {

class ClientCharMatrix;

class Renderer {
public:
    virtual ~Renderer() {}

    virtual void Execute() = 0;

protected:
    Renderer(Config const& config, ClientCharMatrix& matrix)
        : config(config), matrix(matrix) {}

    Config const& config;
    ClientCharMatrix& matrix;

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

}  // namespace client

#endif  // CLIENT_RENDERER_H_

// vim: ts=4:sw=4:sts=4:expandtab
