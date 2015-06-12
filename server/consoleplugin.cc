// Copyright 2015 André Wagner

#include "consoleplugin.h"


namespace server {


ConsolePlugin::ConsolePlugin(Config const& config, string const& plugin_file)
    : config(config)
{
    (void) plugin_file;
}


}  // namespace server

// vim: ts=4:sw=4:sts=4:expandtab
