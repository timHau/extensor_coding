mod extensor;
mod utils;
mod graph;

extern crate nalgebra as na;

use na::{DVector};
// use petgraph::graph::{EdgeIndex, NodeIndex};
use petgraph::visit::GetAdjacencyMatrix;
use petgraph::Graph;

/*
fn compute_walk_sum(
    g: Graph<i32, i32>,
    f_vert: fn(usize) -> DVector<f64>,
    f_edge: fn(usize, usize) -> f64,
) {
    let n = g.node_count();
    let adj_mat = g.adjacency_matrix();
    let mut a: Vec<DVector<f64>> = Vec::new();
    for a_ij in 0..adj_mat.len() {
        let i = a_ij / n; // row index
        let j = a_ij % n; // col index
        if adj_mat.contains(a_ij) {
            let value = f_vert(i) * f_edge(i, j);
            a.push(value);
        } else {
            let zero_vec = DVector::from_element(n, 0.0);
            a.push(zero_vec);
        }
    }
    // let a = DMatrix::from_row_slice(n, n, &g.adjacency_matrix().as_slice());
    let _v = g.node_indices().map(|i| f_vert(i.index()));
}
 */

fn main() {
    let k5 = utils::build_complete_graph(5);
    // println!("{:?}", k5);

    fn f_vert(v: usize) -> DVector<f64> {
        let k = 5;
        utils::vandermonde_vec(v, k)
    }
    ;
    fn f_edge(_e_from: usize, _e_to: usize) -> f64 {
        1.0
    }
    // compute_walk_sum(k5, f_vert, f_edge);

    let graph_path = String::from("src/data/path_graph_10.g6");
    graph::parse_graph6(&graph_path);

    let _vertices = vec![1, 2, 3, 4];
    // let _m = utils::get_vandermonde(vertices, k);
    // println!("{}", m);
    // println!("{}", m.determinant());
}
