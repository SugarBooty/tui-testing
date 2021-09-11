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
    use std::fmt::format;

    use tui::text::Span;
    use tui::widgets::{Block, Borders, Chart, Dataset, Axis};
    // use tui::layout::{Layout, Constraint, Direction};
    use tui::style::{Color, /*Modifier,*/ Style};
    use tui::widgets::BorderType::Rounded;
    use tui::widgets::GraphType::Line;
    use tui::symbols;

    use crate::temps::{TempMaps};

    struct ChartColor {
        color_vec: Vec< Color >,
        itt_count: usize,
    }

    impl ChartColor {
        fn new() -> ChartColor{
            ChartColor {
                color_vec: vec![
                    Color::Red,
                    Color::Yellow,
                    Color::Green,
                    Color::Blue,
                    Color::Cyan,
                ],
                itt_count: 0,
            }
        }
        fn next(&mut self) -> Color {
            let return_color: Color = self.color_vec[self.itt_count];
            self.itt_count += 1;
            if self.itt_count >= self.color_vec.len() {
                self.itt_count = 0;
            }
            return_color
        }
    }

    pub struct CpuGraph<'a> {
        datasets: Vec<Dataset<'a> >,
        x_bounds: [f64; 2],
        y_bounds: [f64; 2],
        x_labels: Vec<Span<'a>>,

    }
    impl<'a> CpuGraph<'a> {
        pub fn new() -> CpuGraph<'a> {
            CpuGraph {
                datasets: vec![],
                x_bounds: [0.0; 2],
                y_bounds: [0.0; 2],
                x_labels: vec![],
            }
        }
        pub fn set_data( &mut self, temp_map: &'a mut TempMaps ) {
            // let mut color_iter = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::White, Color::Cyan, Color::Green, Color::Blue].iter();
            let mut color_iter = ChartColor::new();
            for (name, vec_map) in &temp_map.maps {
                self.datasets.push(
                    Dataset::default()
                    .name(name)
                    .marker(symbols::Marker::Braille)
                    .style(Style::default().fg(color_iter.next()))
                    .data(&vec_map[..])
                )
            }
            self.x_bounds = temp_map.x_minmax;
            self.y_bounds = temp_map.y_minmax;
        }
        fn x_labels(self) -> Vec<Span<'a>>{
            vec![
                Span::styled("first", Style::default())
            ]
        }
        pub fn get_graph(&mut self) -> Chart<'a> {
                Chart::new((*self.datasets).to_vec())
                .block(Block::default().border_type(Rounded).borders(Borders::ALL)
                // .title("CPU Core Temps")
            )
                .x_axis(
                    Axis::default()
                    .bounds(self.x_bounds)
                    // .labels(vec![
                    //     Span::styled(format!("{}", self.x_bounds[0]), Style::default()),
                    //     Span::styled("2", Style::default()),
                    //     Span::styled(format!("{}", self.x_bounds[1]), Style::default()),
                    // ])
                )
                .y_axis(
                    Axis::default()
                    .bounds(self.y_bounds)
                    // FIXME make this use a function to be cleaner and allow logic
                    .labels(vec![
                        Span::styled(format!("{:.0}", self.y_bounds[0]), Style::default()),
                        Span::styled(format!("{:.0}", ((self.y_bounds[1]-self.y_bounds[0]) / 4.0) + self.y_bounds[0]), Style::default()),
                        Span::styled(format!("{:.0}", (((self.y_bounds[1]-self.y_bounds[0]) / 4.0) * 2.0 )+ self.y_bounds[0]), Style::default()),
                        Span::styled(format!("{:.0}", (((self.y_bounds[1]-self.y_bounds[0]) / 4.0) * 3.0 )+ self.y_bounds[0]), Style::default()),
                        Span::styled(format!("{:.0}", self.y_bounds[1]), Style::default()),
                    ])
                )
        }
    }
}