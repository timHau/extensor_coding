extern crate nalgebra as na;

use std::str;

#[derive(Debug)]
struct Graph {
    adj_mat: Box<na::DMatrix<u8>>,
}

impl Graph {
    fn file_n_from_graph6(path_str: &str) -> (Vec<u8>, usize) {
        // read file if it exists
        let mut file = std::fs::read(path_str)
            .expect(".graph6 input file not found");

        let mut n = 0;
        // check if >>graph6<< header is present
        let header = str::from_utf8(&file[..10]).unwrap();
        if header == ">>graph6<<" {
            n = (file[10] - 63) as usize;
            file = file[11..].to_vec();
        } else {
            n = (file[0] - 63) as usize;
            file = file[1..].to_vec();
        }

        if n > 62 {
            let n1 = ((file[0] - 63) as i32) << 12;
            let n2 = ((file[1] - 63) as i32) << 6;
            let n3 = (file[2] - 63) as i32;
            n = (n1 + n2 + n3) as usize;
            file = file[3..].to_vec();
        }

        (file, n)
    }

    fn from_graph6(path_str: &str) -> Self {
        let (file, n) = Self::file_n_from_graph6(path_str);

        let mut buffer = Vec::new();
        file.into_iter()
            .map(|b| b as i32 - 63)
            .for_each(|b| {
                for shift in (0..6).rev() {
                    if (b & 1 << shift) > 0 {
                        buffer.push(1);
                    } else {
                        buffer.push(0);
                    }
                }
            });

        let mut adj_mat: na::DMatrix<u8> = na::DMatrix::zeros(n, n);
        let mut buffer_iter = buffer.iter();
        for i in 1..n {
            for j in 0..i {
                if *(buffer_iter.next().unwrap()) == 1 {
                    adj_mat[i * n + j] = 1;
                    adj_mat[j * n + i] = 1;
                }
            }
        }

        Graph { adj_mat: Box::new(adj_mat) }
    }
}


#[cfg(test)]
mod test {
    use crate::graph::Graph;

    /*
    #[test]
    #[should_panic(expected = ".graph6 input file not found")]
    fn test_graph6_not_found() {
        let graph_path = String::from("src/data/this_is_not_a_file.g6");
        Graph::from_graph6(&graph_path);
    }
     */

    /// returns the adjacency matrix of the n path graph
    fn get_n_path_graph_adj_mat(n: usize) -> na::DMatrix<u8> {
        let mut res: na::DMatrix<u8> = na::DMatrix::zeros(n, n);

        for i in 0..n {
            for j in 0..n {
                if i == j + 1 || i + 1 == j {
                    res[i * n + j] = 1;
                }
            }
        }

        res
    }

    #[test]
    fn test_graph6_header() {
        let graph_with_header = String::from("src/data/test_graphs/path10_with_header.g6");
        let g = Graph::from_graph6(&graph_with_header);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn test_adj_mat() {
        let path_10 = String::from("src/data/test_graphs/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let expect = get_n_path_graph_adj_mat(10);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn test_big_graph() {
        let path_100 = String::from("src/data/test_graphs/path100.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn test_big_graph_with_header() {
        let path_100 = String::from("src/data/test_graphs/path100_with_header.g6");
        let g = Graph::from_graph6(&path_100);
        let expect = get_n_path_graph_adj_mat(100);
        assert_eq!(g.adj_mat, Box::new(expect));
    }
}