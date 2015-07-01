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
    virtual ~Renderer() {}
    virtual bool Running() const = 0;
    virtual vector<UserEvent> GetEvents() const = 0;
    virtual void Update(Matrix const& matrix) const = 0;
};

//
// excpetions
//
struct RendererInitException : public runtime_error {
    explicit RendererInitException(string const& msg) : runtime_error(msg) {}
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
