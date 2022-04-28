use tgraph::{func, Function, MultiGraph};

fn main() {
    println!(
        "{}",
        MultiGraph::new(
            vec![
                func!(x -> 10f64 -(x/5f64)),
                func!(x -> f64::sin(x/2f64).abs() * 4f64),
                func!(x -> x.ln()),
            ],
            80,
            None,
        )
    );
}
