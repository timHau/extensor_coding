extern crate nalgebra as na;

use na::DefaultAllocator;
use na::allocator::Allocator;

#[derive(Debug)]
struct Graph {
    adjacency_matrix: Box<na::DMatrix<u8>>,
}

impl Graph {

   fn from_graph6(path_str: &String) -> Graph {
        // read file if it exists
        let file = std::fs::read(path_str)
            .expect("Input file not found");

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

        let mut adjacency_matrix: na::DMatrix<u8> = na::DMatrix::zeros(n, n);
        let mut buffer_iter = buffer.iter();
        for i in 1..n {
            for j in 0..i {
                if *(buffer_iter.next().unwrap()) == 1 {
                    adjacency_matrix[i * n + j] = 1;
                    adjacency_matrix[j * n + i] = 1;
                }
            }
        }

        Graph { adjacency_matrix: Box::new(adjacency_matrix) }
    }

}


#[cfg(test)]
mod test {
    use crate::graph::Graph;

    #[test]
    fn test_graph6() {
        let graph_path = String::from("src/data/path_graph_10.g6");
        Graph::from_graph6(&graph_path);

    }
}