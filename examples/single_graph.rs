use tgraph::{func, Color, Function, Graph, GraphOptions};

fn main() {
    Graph::with_options_screen(
        func!(|x| 0.005 * (x * x) + 0.1 * x),
        GraphOptions::builder()
            .color(Color::DarkMagenta.into())
            .build(),
    )
    .draw();
}
