#include "pty.h"

#include <cstdlib>
#include <cstdio>
#include <cerrno>
#include <unistd.h>
#include <sys/wait.h>
#include </usr/include/pty.h>
#include <sys/ioctl.h>
#include <fcntl.h>

PTY::PTY(string const& term)
{
    // find correct shell
    char* shell = getenv("SHELL");
    if(!shell) {
        shell = (const char*)"/bin/sh";
    } else {
        shell = &shell[5];
    }

    // fork a new PTY
    pid_t pid;
    if((pid = forkpty(&fd, nullptr, nullptr, nullptr)) < 0) {
        perror("forkpty");
        throw PTYException("Could not for a new PTY.");
    } else if(pid == 0) {
        // child
        if(setenv("TERM", term.c_str(), 1)) {
            perror("putenv");
        }
        if(execlp(shell, "sh", nullptr) == -1) {
            perror("execlp");
            throw PTYException("Could not create the new process.");
        }
    } else {
        // parent
        int flags;
        if((flags = fcntl(fd, F_GETFL, 0)) == -1) {
            flags = 0;
        }
        if(fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -1) {
            perror("fcntl");
            throw new PTYException("Could not set the file status flag.");
        }
    }
}


PTY::~PTY()
{
    close(fd);
}


void 
PTY::Write(vector<uint8_t> const& data) const
{
}


vector<uint8_t> 
PTY::Read() const
{
    return {};
}


// vim: ts=4:sw=4:sts=4:expandtab
