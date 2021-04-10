use super::{extensor::ExTensor, matrix::Matrix};
use num_traits::Zero;
use std::time::Instant;

#[derive(Debug)]
pub struct Graph {
    adj_mat: Box<Matrix<u8>>,
}

/// # Graph
///
/// implementation of the Graph structure used
/// currently a graph is just its adjacency matrix
impl Graph {
    /// ## file_n_from
    ///
    /// given a `path_str` which is the path to a graph6 file as a string, it opens the file and returns
    /// the file with `n` which is the number of vertices in that graph.
    /// It works with .g6 and .s6 files, with and without headers. If the file is not found it will panic.
    pub(crate) fn file_n_from(path_str: &str) -> (Vec<u8>, usize) {
        // read file if it exists
        let mut file = std::fs::read(path_str).expect(".graph6 input file not found");

        let mut _n = 0;

        let has_sparse_header =
            file.len() > 10 && std::str::from_utf8(&file[..11]).unwrap() == ">>sparse6<<";
        let has_graph_header =
            file.len() > 9 && std::str::from_utf8(&file[..10]).unwrap() == ">>graph6<<";
        let is_sparse = file[0] as char == ':' || has_sparse_header;

        if !is_sparse {
            if has_graph_header {
                _n = (file[10] - 63) as usize;
                file = file[11..].to_vec();
            } else {
                _n = (file[0] - 63) as usize;
                file = file[1..].to_vec();
            }
        } else if has_sparse_header {
            _n = (file[12] - 63) as usize;
            file = file[13..].to_vec();
        } else {
            _n = (file[1] - 63) as usize;
            file = file[2..].to_vec();
        }

        if _n > 62 {
            let n1 = ((file[0] - 63) as i32) << 12;
            let n2 = ((file[1] - 63) as i32) << 6;
            let n3 = (file[2] - 63) as i32;
            _n = (n1 + n2 + n3) as usize;
            file = file[3..].to_vec();
        }

        (file, _n)
    }

    /// ## from_graph6
    ///
    /// create a Graph from a .g6 file which is located at `path_str`.
    pub fn from_graph6(path_str: &str) -> Self {
        let (file, n) = Self::file_n_from(path_str);

        let mut buffer = Vec::new();
        file.into_iter().for_each(|b| {
            let v = b as i32 - 63;
            for shift in (0..6).rev() {
                if (v & 1 << shift) > 0 {
                    buffer.push(1);
                } else {
                    buffer.push(0);
                }
            }
        });

        let mut adj_mat = vec![0; n * n];
        let mut buffer_iter = buffer.iter();
        for i in 1..n {
            for j in 0..i {
                if *(buffer_iter.next().unwrap()) == 1 {
                    adj_mat[i * n + j] = 1;
                    adj_mat[j * n + i] = 1;
                }
            }
        }
        let adj_mat = Matrix::new(n, n, adj_mat);

        Graph {
            adj_mat: Box::new(adj_mat),
        }
    }

    fn _from_sparse6(path_str: &str) -> Self {
        let (_file, n) = Self::file_n_from(path_str);

        println!("TODO {}", n);

        Graph {
            adj_mat: Box::new(Matrix::new(0, 0, vec![])),
        }
    }

    /// ## compute_walk_sum
    ///
    /// given an usize `k` and an extensor mapping compute its walk sum.
    /// The mapping is a tuple of closures, where the first element is a function from
    /// a vertex to a extensor and the second element is a function from two vertices (an edge) to
    /// an f64. The walk sum is calculated as
    ///
    /// f(G, ξ) = (1 1 .. 1) A^(k-1) (ξ(v_1) ξ(v_2) ... ξ(v_n))^T
    ///
    pub(crate) fn compute_walk_sum<F, G>(&self, k: usize, mapping: (F, G)) -> ExTensor
    where
        F: Fn(usize) -> ExTensor,
        G: Fn(usize, usize) -> f64,
    {
        let (f_vert, _f_edge) = mapping;

        let transform_start = Instant::now();
        // add extensor coding to vertices and transform back to a matrix
        let a = (*self.adj_mat).add_coding(&f_vert);
        println!(
            "transform took: {} ms",
            transform_start.elapsed().as_millis()
        );

        let b_start = Instant::now();
        let b = (1..(a.ncols() + 1)).map(|i| f_vert(i)).collect::<Vec<_>>();
        println!("b took: {} ms", b_start.elapsed().as_millis());

        let pow_start = Instant::now();
        let mut res = &a * b;
        for _ in 1..k - 1 {
            res = &a * res;
        }
        println!("power took: {} ms", pow_start.elapsed().as_millis());

        res.into_iter().fold(ExTensor::zero(), |acc, v| acc + v)
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::extensor::ExTensor;
    use crate::structure::graph::Graph;
    use crate::structure::matrix_naive::Matrix;
    use crate::utils;
    use num_traits::Zero;

    /*
    #[test]
    #[should_panic(expected = ".graph6 input file not found")]
    fn test_graph6_not_found() {
        let graph_path = String::from("src/data/this_is_not_a_file.g6");
        Graph::from_graph6(&graph_path);
    }
     */

    /*
    /// returns the adjacency matrix of the n path graph
    fn get_n_path_graph_adj_mat(n: usize) -> Matrix<u8> {
        let mut res: Matrix<u8> = Matrix::zeros(n, n);

        for i in 0..n {
            for j in 0..n {
                if i == j + 1 || i + 1 == j {
                    res[(i, j)] = 1;
                }
            }
        }

        res
    }

    #[test]
    fn graph6_header() {
        let graph_with_header = String::from("src/data/test_graphs/path10_with_header.g6");
        let g = Graph::from_graph6(&graph_with_header);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn tutte_graph() {
        let tutte_str = String::from("src/data/test_graphs/tutte_graph.g6");
        let g = Graph::from_graph6(&tutte_str);

        let tutte_mat_file = std::fs::read_to_string("src/data/test_graphs/tutte_mat.txt")
            .expect("could not read tutte_mat.txt");
        let tutte_mat_file = tutte_mat_file.replace('\n', " ");

        let tutte_mat = tutte_mat_file
            .trim()
            .split(" ")
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let t_mat = Matrix::new(46, 46, tutte_mat);
        assert_eq!(g.adj_mat, Box::new(t_mat));
    }

    #[test]
    fn adj_mat() {
        let path_10 = String::from("src/data/test_graphs/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn big_graph() {
        let path_100 = String::from("src/data/test_graphs/path100.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn big_graph_with_header() {
        let path_100 = String::from("src/data/test_graphs/path100_with_header.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(g.adj_mat, Box::new(expect));
    }
    */

    #[test]
    fn compute_walk() {
        let path_10 = String::from("src/data/test_graphs/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let k = 3;
        let res = g.compute_walk_sum(k, utils::create_vandermonde(k));
        let zero = ExTensor::zero();
        assert_ne!(res, zero, "compute walk with vandermonde coding");
    }
}
