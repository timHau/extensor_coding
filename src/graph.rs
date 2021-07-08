#[cfg(feature = "extensor_bitvec")]
use crate::extensor::bitvec::ExTensor;
#[cfg(feature = "extensor_dense_hashmap")]
use crate::extensor::dense_hashmap::ExTensor;

#[cfg(feature = "matrix_naive")]
use crate::matrix::naive::Matrix;
#[cfg(feature = "matrix_sparse_hash")]
use crate::matrix::sparse_hash::Matrix;
#[cfg(feature = "matrix_sparse_triples")]
use crate::matrix::sparse_triples::Matrix;

use crate::utils;
use num_traits::Zero;
use rand::{
    distributions::{Bernoulli, Distribution, Uniform},
    Rng,
};

#[derive(Debug)]
pub struct Graph {
    adj_mat: Box<Matrix<u8>>,
    pub vert_data: Vec<usize>,
}

/// # Graph
///
/// implementation of the Graph structure used
/// currently a graph is just its adjacency matrix
impl Graph {
    /// ## from
    ///
    /// Construct a Graph with `n` vertices given an adjacency matrx `data`
    pub fn from(n: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), n * n, "data must correspond to a square matrix");
        let adj_mat = Box::new(Matrix::new(n, n, data));

        Graph {
            adj_mat,
            vert_data: vec![],
        }
    }

    /// # random_graph
    ///
    /// create a random graph with `n` vertices where each edge has probability `p`.
    pub fn random_graph(n: usize, p: f64) -> Graph {
        assert!(0.0 <= p && p <= 1.0, "Probability must be in (0,1)");

        let mut rng = rand::thread_rng();
        let bernoulli = Bernoulli::new(p).unwrap();
        let data: Vec<u8> = (0..n * n)
            .map(|_| if bernoulli.sample(&mut rng) { 1u8 } else { 0u8 })
            .collect();

        Graph::from(n, data)
    }

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
            vert_data: Vec::with_capacity(n),
        }
    }

    fn from_sparse6(_path_str: &str) -> Self {
        todo!("Not implemented yet");

        /*
        let (file, n) = utils::file_n_from(path_str);

        // let mut buffer = Vec::new();
        file.into_iter().for_each(|b| {
            let v = b as i32 - 63;
            println!("v: {}", v);
        });

        Graph {
            adj_mat: Box::new(Matrix::new(0, 0, vec![])),
            vert_data: Vec::with_capacity(n),
        }
        */
    }

    /// ## from_tsv
    ///
    /// Create a Graph from the given `path_str` tsv file
    /// The second line should be prefixed by a % and contains the number of vertices as the second
    /// entry. So e.g. `% x num_vert num_vert`
    /// All lines that are not prefixed by % are assumend to have the following format:
    /// `from_id to_id`. So the first entry is the index from the "starting" vertex (index starts at `1`)
    /// and the second entry is the index of the "ending" vertex
    pub fn from_tsv(path_str: &str) -> Self {
        let file = std::fs::read_to_string(path_str).expect("file was not found");
        let mut lines = file.lines();

        let dim = lines
            .nth(1)
            .unwrap()
            .split(" ")
            .filter(|v| *v != "%")
            .map(|v| (*v).parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let nrows = dim[1];
        let ncols = dim[2];

        let mut adj_mat = vec![0; nrows * ncols];
        for line in lines {
            if !line.starts_with("%") {
                let splited: Vec<_> = line
                    .split(" ")
                    .take(2)
                    .map(|v| (*v).parse::<i32>().unwrap())
                    .collect();

                let from = (splited[0] - 1) as usize;
                let to = (splited[1] - 1) as usize;

                adj_mat[from * ncols + to] = 1;
            }
        }

        let adj_mat = Matrix::new(nrows, ncols, adj_mat);

        Graph {
            adj_mat: Box::new(adj_mat),
            vert_data: Vec::with_capacity(nrows * ncols),
        }
    }

    /// ## compute_walk_sum
    ///
    /// Given an usize `k` and an extensor mapping compute its walk sum.
    /// The mapping is a tuple of closures, where the first element is a function from
    /// a vertex to a extensor and the second element is a function from two vertices (an edge) to
    /// an f64. The walk sum is calculated as
    ///
    /// f(G, ξ) = (1 1 .. 1) A^(k-1) (ξ(v_1) ξ(v_2) ... ξ(v_n))^T
    ///
    pub fn compute_walk_sum(&self, k: usize, coding: Vec<ExTensor>) -> ExTensor {
        // add extensor coding to vertices and transform back to a matrix
        let a = (*self.adj_mat).add_coding(&coding);

        let b = (0..a.ncols).map(|i| coding[i].clone()).collect::<Vec<_>>();

        let mut res = &a * b;
        for _ in 1..(k - 1) {
            res = &a * res;
        }

        res.into_iter().fold(ExTensor::zero(), |acc, v| acc + v)
    }

    /// ## num_vert
    ///
    /// return the number of vertices / number of columns from the adjacency matrix
    pub(crate) fn num_vert(&self) -> usize {
        self.adj_mat.ncols
    }

    /// ## color_coding
    ///
    /// add a color (number in 1..=k) to every vertex
    /// the colors are stored in the `vert_data` field
    pub(crate) fn color_coding(&self, k: usize) -> Self {
        let num_verts = (*self.adj_mat).ncols;
        let rng = rand::thread_rng();
        let colors: Vec<_> = rng
            .sample_iter(&Uniform::new(1, k + 1))
            .take(num_verts)
            .collect();

        Graph {
            adj_mat: self.adj_mat.clone(),
            vert_data: colors,
        }
    }

    pub(crate) fn neighbors_of(&self, i: usize) -> Vec<usize> {
        self.adj_mat.neighbors_of(i)
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
        Graph::from_graph6("src/data/this_is_not_a_file.g6");
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
        let g = Graph::from_graph6("src/data/path10_with_header.g6");
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(
            *g.adj_mat, expect,
            "Graph6 file with header should be read correctly"
        );
    }

    /*
    #[test]
    fn sparse6() {
        let _g = Graph::from_sparse6("src/data/path10.s6");
    }
    */

    #[test]
    fn tutte_graph() {
        let g = Graph::from_graph6("src/data/tutte_graph.g6");

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
        let g = Graph::from_graph6("src/data/path10.g6");
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(*g.adj_mat, expect, "10 path graph should be read correctly");
    }

    #[test]
    fn big_graph() {
        let g = Graph::from_graph6("src/data/path100.g6");
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(
            *g.adj_mat, expect,
            "100 path graph should be read correctly"
        );
    }

    #[test]
    fn big_graph_with_header() {
        let g = Graph::from_graph6("src/data/path100_with_header.g6");
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(
            *g.adj_mat, expect,
            "100 path graph with header should be read correctly"
        );
    }

    #[test]
    fn read_tsv() {
        let g = Graph::from_tsv("src/data/out.brunson_southern-women_southern-women");
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
    fn read_tsv_2() {
        let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");
        let expect = Matrix::new(
            136,
            5,
            vec![
                1, 1, 0, 0, 0, // 1
                1, 1, 1, 0, 0, // 2
                1, 0, 0, 0, 0, // 3
                1, 0, 1, 0, 0, // 4
                0, 0, 0, 1, 0, // 5
                0, 0, 0, 0, 1, // 6
                1, 0, 0, 0, 0, // 7
                1, 0, 1, 0, 0, // 8
                0, 0, 0, 1, 0, // 9
                1, 0, 0, 0, 1, // 10
                0, 0, 0, 1, 0, // 11
                0, 0, 0, 1, 0, // 12
                1, 0, 0, 0, 0, // 13
                0, 0, 1, 0, 0, // 14
                0, 0, 1, 0, 0, // 15
                0, 0, 0, 1, 0, // 16
                1, 0, 0, 0, 0, // 17
                0, 0, 0, 1, 0, // 18
                0, 0, 0, 1, 0, // 19
                0, 0, 0, 1, 0, // 20
                1, 0, 0, 0, 0, // 21
                0, 0, 0, 1, 0, // 22
                0, 0, 0, 1, 0, // 23
                1, 0, 0, 0, 0, // 24
                1, 0, 0, 0, 0, // 25
                1, 0, 0, 0, 0, // 26
                1, 0, 0, 0, 1, // 27
                1, 0, 0, 0, 0, // 28
                0, 0, 0, 1, 0, // 29
                1, 0, 0, 0, 0, // 30
                1, 1, 1, 0, 0, // 31
                0, 0, 0, 0, 1, // 32
                0, 0, 0, 1, 0, // 33
                1, 0, 0, 0, 0, // 34
                1, 0, 0, 0, 0, // 35
                0, 1, 0, 0, 0, // 36
                0, 1, 0, 0, 0, // 37
                0, 0, 0, 1, 1, // 38
                0, 0, 1, 0, 0, // 39
                0, 1, 0, 0, 0, // 40
                1, 0, 1, 0, 0, // 41
                0, 0, 0, 1, 0, // 42
                0, 1, 0, 0, 0, // 43
                0, 0, 0, 1, 0, // 44
                1, 0, 0, 0, 1, // 45
                1, 0, 0, 0, 0, // 46
                0, 0, 0, 1, 0, // 47
                0, 0, 0, 0, 1, // 48
                0, 0, 0, 1, 0, // 49
                0, 1, 0, 0, 0, // 50
                0, 0, 0, 1, 0, // 51
                0, 0, 0, 1, 0, // 52
                1, 0, 0, 0, 0, // 53
                0, 0, 0, 1, 0, // 54
                1, 0, 1, 0, 0, // 55
                0, 0, 1, 0, 0, // 56
                0, 0, 0, 1, 0, // 57
                0, 1, 0, 0, 0, // 58
                1, 0, 0, 0, 0, // 59
                0, 0, 1, 0, 0, // 60
                0, 0, 0, 1, 0, // 61
                1, 0, 0, 0, 0, // 62
                0, 0, 0, 1, 0, // 63
                1, 0, 0, 0, 0, // 64
                1, 0, 0, 0, 0, // 65
                0, 0, 0, 1, 0, // 66
                0, 0, 0, 1, 0, // 67
                0, 0, 0, 1, 0, // 68
                0, 0, 0, 1, 0, // 69
                1, 0, 0, 0, 0, // 70
                1, 0, 0, 0, 0, // 71
                0, 0, 0, 1, 0, // 72
                1, 0, 0, 0, 0, // 73
                0, 1, 0, 0, 0, // 74
                0, 0, 0, 1, 0, // 75
                1, 0, 0, 0, 0, // 76
                0, 0, 1, 0, 0, // 77
                0, 0, 0, 1, 0, // 78
                0, 0, 0, 1, 0, // 79
                1, 0, 0, 0, 0, // 80
                0, 0, 0, 1, 0, // 81
                1, 0, 0, 0, 0, // 82
                0, 0, 0, 1, 0, // 83
                1, 0, 1, 0, 0, // 84
                0, 0, 0, 1, 0, // 85
                1, 0, 0, 0, 0, // 86
                0, 0, 0, 1, 0, // 87
                0, 0, 0, 1, 0, // 88
                0, 1, 1, 0, 0, // 89
                0, 0, 0, 1, 0, // 90
                1, 0, 0, 0, 0, // 91
                1, 0, 0, 0, 0, // 92
                1, 0, 0, 0, 0, // 93
                1, 0, 0, 0, 0, // 94
                0, 0, 0, 1, 0, // 95
                1, 0, 0, 0, 0, // 96
                0, 0, 0, 1, 0, // 97
                0, 1, 0, 0, 0, // 98
                0, 0, 1, 0, 0, // 99
                0, 0, 0, 1, 0, // 100
                0, 0, 1, 0, 0, // 101
                1, 0, 0, 0, 0, // 102
                1, 0, 0, 0, 0, // 103
                0, 0, 0, 1, 0, // 104
                0, 1, 1, 0, 0, // 105
                1, 1, 0, 1, 0, // 106
                1, 0, 0, 0, 0, // 107
                0, 0, 0, 1, 0, // 108
                1, 0, 0, 0, 0, // 109
                1, 0, 0, 0, 0, // 110
                0, 0, 0, 1, 0, // 111
                0, 0, 0, 0, 1, // 112
                0, 0, 0, 1, 0, // 113
                1, 0, 0, 0, 0, // 114
                1, 0, 0, 0, 0, // 115
                1, 0, 0, 0, 0, // 116
                1, 0, 0, 0, 0, // 117
                0, 0, 1, 0, 0, // 118
                1, 0, 0, 0, 0, // 119
                1, 0, 0, 0, 0, // 120
                0, 0, 0, 1, 0, // 121
                1, 0, 0, 0, 0, // 122
                0, 0, 0, 0, 1, // 123
                0, 1, 0, 0, 0, // 124
                1, 0, 0, 1, 0, // 125
                1, 1, 1, 1, 0, // 126
                0, 0, 0, 1, 0, // 127
                0, 0, 0, 1, 0, // 128
                0, 0, 0, 1, 1, // 129
                0, 0, 1, 0, 0, // 130
                1, 0, 0, 0, 0, // 131
                0, 0, 0, 1, 0, // 132
                0, 0, 0, 1, 0, // 133
                0, 1, 0, 0, 0, // 134
                1, 0, 0, 0, 0, // 135
                1, 0, 1, 0, 0, // 136
            ],
        );
        assert_eq!(*g.adj_mat, expect, "tsv reads correct adj_mat");
    }

    #[test]
    fn compute_walk() {
        let g = Graph::from_graph6("src/data/path10.g6");
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
        let g = Graph::from_graph6("src/data/path10.g6");
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
        let g = Graph::from_graph6("src/data/path3.g6");
        let k = 5;
        let res = g.compute_walk_sum(k, utils::create_vandermonde(g.num_vert(), k));
        assert_eq!(
            res.is_zero(),
            true,
            "compute walk with vandermonde coding should be zero"
        );
    }
}
