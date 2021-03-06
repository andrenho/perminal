#include "pty.h"

#include <cstdlib>
#include <cstdio>
#include <cerrno>
#include <cstring>
#include <unistd.h>
#include <sys/wait.h>
#include </usr/include/pty.h>
#include <sys/ioctl.h>
#include <fcntl.h>

#include "debug.h"

PTY::PTY(string const& term)
{
    // find correct shell
    const char* shell = getenv("SHELL");
    if(!shell) {
        shell = "/bin/sh";
    }
    D("Using the following shell: %s", shell);

    // fork a new PTY
    pid_t pid;
    D("Forking a new PTY...");
    if((pid = forkpty(&fd, nullptr, nullptr, nullptr)) < 0) {
        perror("forkpty");
        throw PluginException("Could not for a new PTY.");
    } else if(pid == 0) {
        /*
         * child
         */

        // set term
        if(setenv("TERM", term.c_str(), 1)) {
            perror("putenv");
        }
        PrintMOTD();
        
        // initialize shell
        if(execlp(shell, "", nullptr) == -1) {
            perror("execlp");
            throw PluginException("Could not create the new process.");
        }
    } else {
        /*
         * parent
         */

        // set file descriptor as non-blocking
        int flags;
        if((flags = fcntl(fd, F_GETFL, 0)) == -1) {
            flags = 0;
        }
        if(fcntl(fd, F_SETFL, flags | O_NONBLOCK) == -1) {
            perror("fcntl");
            throw new PluginException("Could not set the file status flag.");
        }
        D("(parent) PTY file descriptor set to O_NONBLOCK");
    }
}


PTY::~PTY()
{
    close(fd);
}


void
PTY::Write(const uint8_t* data, int n) const
{
    if(write(fd, data, n) == -1) {
        perror("write");
        throw PluginException("There was an error writing to the PTY");
    }
}


int 
PTY::Read(uint8_t* data, int max_sz) const
{
    int nread = read(fd, data, max_sz);
    
    if(nread == -1) {
        switch(errno) {
        case EAGAIN:
            return 0;   // no data from socket (socket is O_NONBLOCK)
        case EIO:
            return -1;  // the connection was cut
        default:
            perror("read");
            throw PluginException("There was an error reading from the PTY.");
        }
    } else if(nread == 0) {
        return -1;  // the connection ended
    }

    return nread;
}

    
void 
PTY::Resize(int w, int h) const
{
    struct winsize ws = { h, w, 0, 0 };
    if(ioctl(fd, TIOCSWINSZ, &ws) < 0) {
        D("Couldn't set window size.\n");
    }
}


void
PTY::PrintMOTD() const
{
#ifndef DEBUG
    printf("`perminal` aims to be a very fast, small, highly compliant and highly\n");
    printf("configurable terminal emulator for various operating systems, with multiple\n");
    printf("front-ends and multiple-backends.\n");
    printf("\n");
    printf("Right now, it is niether of these things. This is a very alpha release, so DO\n");
    printf("NOT USE IN PRODUCTION ENVIRONMENTS.\n");
    printf("\n");
#endif
}


// vim: ts=4:sw=4:sts=4:expandtab
