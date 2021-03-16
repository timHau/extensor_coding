mod structures;
mod utils;

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
    // println!("{:?}", k5);
}
