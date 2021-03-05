extern crate nalgebra as na;

use petgraph::Graph;
use na::DMatrix;


pub fn build_complete_graph(n: i32) -> Graph<i32, i32> {
    let mut g = Graph::<i32, i32>::new();

    let mut nodes = Vec::new();
    for i in 0..n {
        let node = g.add_node(i);
        nodes.push(node);
    }

    for node in nodes.iter() {
        for node_b in nodes.iter() {
            if node != node_b {
                g.add_edge(*node, *node_b, 1);
            }
        }
    }

    g
}

pub fn get_vandermonde(vertices: Vec<i64>, k: usize) -> DMatrix<f64> {
    let n = vertices.len();
    DMatrix::from_fn(n, k, |i, j| -> f64 {
        (vertices[i]).pow(j as u32) as f64
    })
}
