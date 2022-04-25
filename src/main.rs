use tgraph::{func, Character, Function, Graph, GraphOptions, MultiGraph};

fn main() {
    // println!("{:?}", func!(x [u32] -> 5 * x => [u32]));
    // Graph::with_options_screen(
    //     |x| 0.005 * (x * x) + 0.1 * x,
    //     GraphOptions::builder()
    //         .color(Color::DarkMagenta.into())
    //         .build(),
    // )
    // .draw();

    // MultiGraph::new_screen(vec![func!(x -> (x/2f64).sin() * 4f64), func!(x -> x.ln())]).draw();

    println!(
        "{}",
        MultiGraph::new_screen(
            vec![
                // Function::new(first as fn(f64) -> f64),
                // Function::new(second as fn(f64) -> f64),
                // func!(x -> 15f64 -(x/4f64)),
                func!(x -> f64::sin(x/2f64).abs() * 4f64),
                func!(x -> x.ln()),
                // func!(x -> x.exp()),
            ],
            // 80,
            // None,
        )
    )
}
