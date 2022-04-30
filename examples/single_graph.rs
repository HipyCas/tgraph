use tgraph::{func, Color, Graph, GraphOptions};

/// Generate a graph with a single function in it
/// the func! macro defines the function we want to display
/// `Graph::with_options_screen` allow us to define how we want our graph to appear with the second parameter
fn main() {
    Graph::with_options_screen(
        func!(|x| 0.005 * (x * x) + 0.1 * x),
        GraphOptions::builder()
            .color(Color::DarkMagenta.into())
            .build(),
    )
    .draw();
}
