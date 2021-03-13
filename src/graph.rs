extern crate nalgebra as na;

use std::str;

#[derive(Debug)]
struct Graph {
    adj_mat: Box<na::DMatrix<u8>>,
}

impl Graph {

    fn file_n_from_graph6(path_str: &str) -> (Vec<u8>, usize) {
        // TODO handle graphs with more than 62 vertices

        // read file if it exists
        let mut file = std::fs::read(path_str)
            .expect(".graph6 input file not found");

        let mut n = 0;
        // check if >>graph6<< header is present
        let header = str::from_utf8(&file[..10]).unwrap();
        if header == ">>graph6<<" {
            n = (file[10]- 63) as usize;
            file = file[10..].to_vec();
        } else {
            n = (file[0] - 63) as usize
        }

        (file, n)
    }

    fn from_graph6(path_str: &str) -> Self {
        let (file, n) = Self::file_n_from_graph6(path_str);

        let mut buffer = Vec::new();
        file.into_iter()
            .skip(1)
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

    fn get_path10_adj_mat() -> na::DMatrix<u8> {
        na::DMatrix::from_row_slice(10, 10, &[
            0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
            1, 0, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 1, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 1, 0, 1, 0,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        ])
    }

    #[test]
    fn test_graph6_header() {
        let graph_with_header = String::from("src/data/test_graphs/path10_with_header.g6");
        let g = Graph::from_graph6(&graph_with_header);
        let expect = get_path10_adj_mat();
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn test_adj_mat() {
        let path_10 = String::from("src/data/test_graphs/path10.g6");
        let g = Graph::from_graph6(&path_10);
        let expect = get_path10_adj_mat();
        assert_eq!(g.adj_mat, Box::new(expect));
    }

    #[test]
    fn test_big_graph() {
        let path_100 = String::from("src/data/test_graphs/path100.g6");
//        let g = Graph::from_graph6(&path_100);
    }
}