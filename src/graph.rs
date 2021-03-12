extern crate nalgebra as na;

pub fn parse_graph6(path_str: &String) {
    // read file if it exists
    let file = std::fs::read(path_str)
        .expect("Input file not found");

    // TODO handle graphs with more than 62 vertices
    let n = (file[0] - 63) as usize;

    let mut buffer  = Vec::new();
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

    let mut adjacency_matrix = na::DMatrix::zeros(n, n);
    let mut  buffer_iter = buffer.iter();
    for i in 1..n {
        for j in 0..i {
            if *(buffer_iter.next().unwrap()) == 1 {
                adjacency_matrix[i * n + j] = 1.0;
                adjacency_matrix[j * n + i] = 1.0;
            }
        }
    }

    println!("{}", adjacency_matrix);
}