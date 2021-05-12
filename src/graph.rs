#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

#[cfg(feature = "matrix_naive")]
use crate::matrix::naive::Matrix;
#[cfg(feature = "matrix_naive_parallel")]
use crate::matrix::naive_parallel::Matrix;
#[cfg(feature = "matrix_sparse_hash")]
use crate::matrix::sparse_hash::Matrix;
#[cfg(feature = "matrix_sparse_triples")]
use crate::matrix::sparse_triples::Matrix;

use crate::utils;
use num_traits::Zero;

#[derive(Debug)]
pub struct Graph {
    adj_mat: Box<Matrix<u8>>,
}

/// # Graph
///
/// implementation of the Graph structure used
/// currently a graph is just its adjacency matrix
impl Graph {
    /// ## from_graph6
    ///
    /// create a Graph from a .g6 file which is located at `path_str`.
    pub fn from_graph6(path_str: &str) -> Self {
        let (file, n) = utils::file_n_from(path_str);

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
        let (_file, n) = utils::file_n_from(path_str);

        println!("TODO {}", n);

        Graph {
            adj_mat: Box::new(Matrix::new(0, 0, vec![])),
        }
    }

    fn from_tsv(path_str: &str) -> Self {
        let mut file = std::fs::read_to_string(path_str).expect("file was not found");
        let mut lines = file.lines();

        let n = lines
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|v| *v != "%")
            .map(|v| (*v).parse::<i32>().unwrap())
            .collect::<Vec<_>>()[1] as usize;

        let mut adj_mat = vec![0; n * n];
        for line in lines {
            if !line.starts_with("%") {
                let splited: Vec<_> = line
                    .split(" ")
                    .take(2)
                    .map(|v| (*v).parse::<i32>().unwrap())
                    .collect();

                let from = (splited[0] - 1) as usize;
                let to = (splited[1] - 1) as usize;
                adj_mat[to + n * from] = 1;
            }
        }

        let adj_mat = Matrix::new(n, n, adj_mat);

        Graph {
            adj_mat: Box::new(adj_mat),
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
    pub(crate) fn compute_walk_sum(&self, k: usize, coding: Vec<ExTensor>) -> ExTensor {
        // add extensor coding to vertices and transform back to a matrix
        let a = (*self.adj_mat).add_coding(&coding);

        let b = (0..a.ncols())
            .map(|i| coding[i].clone())
            .collect::<Vec<_>>();

        let mut res = &a * b;
        for _ in 1..(k - 1) {
            res = &a * res;
        }

        res.into_iter().fold(ExTensor::zero(), |acc, v| acc + v)
    }

    pub(crate) fn num_vert(&self) -> usize {
        self.adj_mat.ncols()
    }
}

impl std::clone::Clone for Graph {
    fn clone(&self) -> Self {
        Graph {
            adj_mat: Box::new(*self.adj_mat.clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "extensor_bitvec")]
    use crate::extensor::bitvec::ExTensor;
    #[cfg(feature = "extensor_dense_hashmap")]
    use crate::extensor::dense_hashmap::ExTensor;
    #[cfg(feature = "matrix_naive")]
    use crate::matrix::naive::Matrix;
    #[cfg(feature = "matrix_naive_parallel")]
    use crate::matrix::naive_parallel::Matrix;
    #[cfg(feature = "matrix_sparse_hash")]
    use crate::matrix::sparse_hash::Matrix;
    #[cfg(feature = "matrix_sparse_triples")]
    use crate::matrix::sparse_triples::Matrix;

    use crate::graph::Graph;
    use crate::utils;
    use num_traits::Zero;

    #[test]
    #[should_panic(expected = ".graph6 input file not found")]
    fn test_graph6_not_found() {
        let graph_path = String::from("src/data/this_is_not_a_file.g6");
        Graph::from_graph6(&graph_path);
    }

    /// returns the adjacency matrix of the n path graph
    fn get_n_path_graph_adj_mat(n: usize) -> Matrix<u8> {
        let mut res = Vec::with_capacity(n * n);
        res.reserve(n * n);

        for i in 0..n {
            for j in 0..n {
                if i == j + 1 || i + 1 == j {
                    res.push(1);
                } else {
                    res.push(0);
                }
            }
        }

        Matrix::new(n, n, res)
    }

    #[test]
    fn graph6_header() {
        let graph_with_header = String::from("src/data/path10_with_header.g6");
        let g = Graph::from_graph6(&graph_with_header);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(
            *g.adj_mat, expect,
            "Graph6 file with header should be read correctly"
        );
    }

    #[test]
    fn tutte_graph() {
        let tutte_str = String::from("src/data/tutte_graph.g6");
        let g = Graph::from_graph6(&tutte_str);

        let tutte_mat_file = std::fs::read_to_string("src/data/tutte_mat.txt")
            .expect("could not read tutte_mat.txt");
        let tutte_mat_file = tutte_mat_file.replace('\n', " ");

        let tutte_mat = tutte_mat_file
            .trim()
            .split(" ")
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        let t_mat = Matrix::new(46, 46, tutte_mat);
        assert_eq!(*g.adj_mat, t_mat, "Tutte Graph should be read correctly");
    }

    #[test]
    fn adj_mat() {
        let path_10 = String::from("src/data/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(*g.adj_mat, expect, "10 path graph should be read correctly");
    }

    #[test]
    fn big_graph() {
        let path_100 = String::from("src/data/path100.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(
            *g.adj_mat, expect,
            "100 path graph should be read correctly"
        );
    }

    #[test]
    fn big_graph_with_header() {
        let path_100 = String::from("src/data/path100_with_header.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(
            *g.adj_mat, expect,
            "100 path graph with header should be read correctly"
        );
    }

    #[test]
    fn read_tsv() {
        let tsv = String::from("src/data/out.brunson_southern-women_southern-women");
        let g = Graph::from_tsv(&tsv);
        let expect = Matrix::new(
            5,
            5,
            vec![
                1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1,
            ],
        );
        assert_eq!(*g.adj_mat, expect, "tsv reads correct adj_mat");
    }

    #[test]
    fn compute_walk() {
        let path_10 = String::from("src/data/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let k = 3;
        let res = g.compute_walk_sum(k, utils::create_vandermonde(g.num_vert(), k));
        let zero = ExTensor::zero();
        assert_ne!(
            res, zero,
            "compute walk with vandermonde coding should not be zero"
        );
    }

    #[test]
    fn compute_walk_2() {
        let path_10 = String::from("src/data/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let k = 5;
        let res = g.compute_walk_sum(k, utils::create_vandermonde(g.num_vert(), k));
        let zero = ExTensor::zero();
        assert_ne!(
            res, zero,
            "compute walk with vandermonde coding should not be zero"
        );
    }

    #[test]
    fn compute_walk_3() {
        let path_10 = String::from("src/data/path3.g6");
        let g = Graph::from_graph6(&path_10);
        let k = 5;
        let res = g.compute_walk_sum(k, utils::create_vandermonde(g.num_vert(), k));
        assert_eq!(
            res.is_zero(),
            true,
            "compute walk with vandermonde coding should be zero"
        );
    }
}
