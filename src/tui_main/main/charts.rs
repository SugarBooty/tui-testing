/*
This module generates the cpu graph pbjects, it has functions to parse data into
TUI Chart objects that can be placed in the draw function in the loop. 

To use: instantiate a CpuGraph struct with temp_handler (this is found in the get_data module):
    let cpu_graph_handler = CpuGraph::new();

insert the tempmap into the object for parsing:
    cpu_raph_handler.set_data(temp_handler);

after that, running the get_graph method will return a Chart object used in the draw loop
    let cpu_graph = cpu_graph_handler.get_graph();
    ...
    f.render_widget(cpu_graph, f.size());
*/

pub mod cpu_graph {
    use tui::widgets::{Block, Borders, Chart, Dataset, Axis};
    // use tui::layout::{Layout, Constraint, Direction};
    use tui::style::{Color, /*Modifier,*/ Style};
    use tui::symbols;

    use crate::temps::{TempMaps};

    pub struct CpuGraph<'a> {
        datasets: Vec<Dataset<'a> >,
        x_bounds: [f64; 2],
        y_bounds: [f64; 2],
    }
    impl<'a> CpuGraph<'a> {
        pub fn new() -> CpuGraph<'a> {
            CpuGraph {
                datasets: vec![],
                x_bounds: [0.0; 2],
                y_bounds: [0.0; 2],
                
            }
        }
        pub fn set_data( &mut self, temp_map: &'a mut TempMaps ) {
            let mut color_iter = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::White, Color::Cyan, Color::Green, Color::Blue].iter();
            for (name, vec_map) in &temp_map.maps {
                self.datasets.push(
                    Dataset::default()
                    .name(name)
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(*color_iter.next().unwrap()))
                    .data(&vec_map[..])
                )
            }
            self.x_bounds = temp_map.x_minmax;
            self.y_bounds = temp_map.y_minmax;
        }
        pub fn get_graph(&mut self) -> Chart<'a> {
                Chart::new((*self.datasets).to_vec())
                .block(Block::default().borders(Borders::ALL)
                .title("CPU Core Temps"))
                .x_axis(
                    Axis::default()
                    .bounds(self.x_bounds)
                )
                .y_axis(
                    Axis::default()
                    .bounds(self.y_bounds)
                )
        }
    }
}