mod utils;

use petgraph::Graph;
use petgraph::graph::NodeIndex;

fn compute_walk_sum(g: Graph<i32, i32>, _f: fn(NodeIndex) -> f64) {
    g.node_indices().for_each(|i| {
        println!("{}", i.index());
    });
}

fn main() {
    let k5 = utils::build_complete_graph(5);
    // println!("{:?}", k5);

    fn f(_n: NodeIndex) -> f64 {
        0.0
    }
    compute_walk_sum(k5, f);

    let vertices = vec![1, 2, 3, 4];
    let k = 4;
    let _m = utils::get_vandermonde(vertices, k);
    // println!("{}", m);
    // println!("{}", m.determinant());
}
