use extensor_coding::{algorithm, graph::Graph};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let g = Graph::from_tsv("src/data/out.brunson_southern-women_southern-women");
    let k = 4;
    let eps = 0.01;
    let res = algorithm::c(g, k, eps);
    println!("res: {}", res);

    // // let g = Graph::from_tsv("src/data/out.arenas-jazz");
    // let g = Graph::from_tsv("src/data/out.brunson_revolution_revolution");
    // // let g = Graph::from_graph6("src/data/path10.g6");
    // let k = 4;

    // let path = Path::new("debug/debug.txt");
    // let mut values = vec![];
    // for eps in vec![0.2].into_iter() {
    //     let value = algorithm::c_values_t_test(g.clone(), k, eps);
    //     values.push(value);
    // }

    // let mut file = match File::create(&path) {
    //     Err(why) => panic!("could not create file: {}", why),
    //     Ok(file) => file,
    // };

    // let mut values_as_string: Vec<String> = vec![];
    // for value in values.iter() {
    //     let as_string: Vec<String> = value.iter().map(|v| v.to_string()).collect();
    //     values_as_string.push(as_string.join(", "));
    // }
    // match file.write_all(values_as_string.join("\n").as_bytes()) {
    //     Err(why) => panic!("could not write: {}", why),
    //     Ok(_) => println!("successfully wrote to debug.txt"),
    // }
}
