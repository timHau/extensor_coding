use extensor_coding::{algorithm, graph::Graph};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // let g = Graph::from_tsv("src/data/out.arenas-jazz");
    let g = Graph::from_graph6("src/data/path10.g6");
    let k = 4;
    let eps = 0.2;

    let values = algorithm::c_values(g, k, eps);

    let path = Path::new("debug/debug.txt");

    let mut file = match File::create(&path) {
        Err(why) => panic!("could not create file: {}", why),
        Ok(file) => file,
    };

    let values_as_string: Vec<String> = values.iter().map(|v| v.to_string()).collect();
    match file.write_all(values_as_string.join(", ").as_bytes()) {
        Err(why) => panic!("could not write: {}", why),
        Ok(_) => println!("successfully wrote to debug.txt"),
    }
}
