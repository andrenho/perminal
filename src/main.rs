mod plugin;
mod echo;
mod terminal;
mod renderer;
mod curses_renderer;
mod matrix;
mod userevent;
mod command;

use renderer::Renderer;
use curses_renderer::CursesRenderer;

fn main() {
    let plugin = echo::Echo::new();
    let mut terminal = terminal::Terminal::new(&plugin);
    let renderer = curses_renderer::CursesRenderer::new();

    while terminal.is_alive() && renderer.is_running() {

        // user input
        for k in renderer.get_user_input().iter() {
            match terminal.input(k) {
                Ok(_) => (),
                Err(msg) => panic!(msg),
            }
        }

        // output to user
        terminal.parse_plugin_output();
        renderer.update(&mut terminal.matrix);
    }
}

// vim: ts=4:sw=4:sts=4:expandtab
