pub mod main {
    use std::io;
    // use sysinfo::System;
    // use termion::raw::IntoRawMode;
    use tui::Terminal;
    use tui::backend::TermionBackend;
    // use tui::layout::{Layout, Constraint, Direction};

    use crate::temps::{TempMaps};

    mod charts;
    use charts::cpu_graph::CpuGraph;

    pub struct Display<'a>{
        terminal: tui::Terminal<tui::backend::TermionBackend<std::io::Stdout>>,
        temp_handler: &'a mut TempMaps,
    }
    impl<'a> Display<'_> {
        // sets vars needed for TUI to draw to terminal
        pub fn new(temp_handler: &'a mut TempMaps) -> Display<'a> {
            let stdout = io::stdout();
            let backend = TermionBackend::new(stdout);
            let terminal = Terminal::new(backend).unwrap();
            Display {
                terminal: terminal,
                temp_handler: temp_handler,
            }
        }
        // generates and pushes the frames to the temrminal
        pub fn draw_display(&mut self) {
            self.temp_handler.refresh_temps(); // refreshes the temp data

            let mut cpu_graph_handler = CpuGraph::new(); // makes a new cpu_graph instance
            cpu_graph_handler.set_data(self.temp_handler); // parses the data into the graph

            self.terminal.draw(|f| {
                let cpu_graph = cpu_graph_handler.get_graph(); // oututs the widget data
                f.render_widget(cpu_graph, f.size());
            }).unwrap();
        }
    }
}