#ifndef PLUGIN_H
#define PLUGIN_H

#include <vector>
using namespace std;

class Plugin {
public:
    virtual ~Plugin() {}
    virtual void Write(const uint8_t* data, int n) const = 0;
    virtual int Read(uint8_t* data, int max_sz) const = 0;
    virtual void Resize(int w, int h) const { (void)w; (void)h; }
};

//
// exceptions
//
struct PluginException : public runtime_error {
    explicit PluginException(string const& msg) : runtime_error(msg) {}
};

#endif

// vim: ts=4:sw=4:sts=4:expandtab
