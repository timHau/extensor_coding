extern crate nalgebra as na;
use na::DMatrix;
use petgraph::Graph;
use std::fmt::Debug;

fn get_vandermonde(vertices: Vec<i64>, k: usize) -> DMatrix<f64> {
    let n = vertices.len();
    DMatrix::from_fn(n, k, |i, j| -> f64 {
        (vertices[i]).pow(j as u32) as f64
    })
}

fn build_complete_graph(n: i32) -> Graph<i32, i32> {
    let mut g = Graph::<i32, i32>::new();

    let mut nodes = Vec::new();
    for i in 0..n {
        let node = g.add_node(i);
        nodes.push(node);
    }

    let mut edges = Vec::new();
    for node in nodes.iter() {
        for node_b in nodes.iter() {
            if node != node_b {
                
            }
        }
    }

    g
}

fn main() {
    let k3 = build_complete_graph(3);

    let vertices = vec![1, 2, 3, 4];
    let k = 4;
    let m = get_vandermonde(vertices, k);
    println!("{}", m);
    println!("{}", m.determinant());
}
