mod utils;

fn main() {
    let k5 = utils::build_complete_graph(5);
    println!("{:?}", k5);

    let vertices = vec![1, 2, 3, 4];
    let k = 4;
    let m = utils::get_vandermonde(vertices, k);
    println!("{}", m);
    println!("{}", m.determinant());
}
