mod config;
mod plugin;
mod pty;
mod terminfo;
mod terminfo_xterm256;
mod terminal;
mod renderer;
mod x11_renderer;
mod matrix;
mod userevent;
mod command;

use renderer::Renderer;
use x11_renderer::X11Renderer;

fn main() {
    let cfg = config::Config::new();
    let plugin = pty::PTY::new();
    let mut terminal = terminal::Terminal::new(&cfg, plugin);
    {
        let renderer = x11_renderer::X11Renderer::new();

        while terminal.is_alive() && renderer.is_running() {

            // user input
            for k in renderer.get_user_input().iter() {
                match terminal.user_input(k) {
                    Ok(_) => (),
                    Err(msg) => panic!(msg),
                }
            }

            // output to user
            terminal.parse_plugin_output();
            renderer.update(&mut terminal.matrix);
        }
    }

    terminal.print_debug_info();
}

// vim: ts=4:sw=4:sts=4:expandtab
