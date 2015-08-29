#ifndef RENDERER_H
#define RENDERER_H

#include <stdexcept>
#include <string>
#include <vector>
using namespace std;

#include "userevent.h"
#include "matrix.h"

class Renderer {
public:
    explicit Renderer(Matrix const& matrix) : matrix(matrix) {}
    virtual ~Renderer() {}
    virtual bool Running() const = 0;
    virtual UserEvent GetEvent() const = 0;
    virtual void Update() const = 0;

protected:
    Matrix const& matrix;
};

//
// exceptions
//
struct RendererInitException : public runtime_error {
    explicit RendererInitException(string const& msg) : runtime_error(msg) {}
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
