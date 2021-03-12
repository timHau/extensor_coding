extern crate nalgebra as na;

use na::DefaultAllocator;
use na::allocator::Allocator;

#[derive(Debug)]
struct Graph {
    adj_mat: Box<na::DMatrix<u8>>,
}

impl Graph {
    fn from_graph6(path_str: &String) -> Graph {
        // read file if it exists
        let file = std::fs::read(path_str)
            .expect(".graph6 input file not found");

        // TODO handle graphs with more than 62 vertices
        let n = (file[0] - 63) as usize;

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

    #[test]
    #[should_panic(expected = ".graph6 input file not found")]
    fn test_graph6_not_found() {
        // test that
        let graph_path = String::from("src/data/this_is_not_a_file.g6");
        Graph::from_graph6(&graph_path);
    }

    #[test]
    fn test_adj_mat() {
        let path_10 = String::from("src/data/test_graph_path10.g6");
        let g = Graph::from_graph6(&path_10);
        let expect = na::DMatrix::from_row_slice(10, 10, &[
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
        ]);
        assert_eq!(g.adj_mat, Box::new(expect));
    }
}