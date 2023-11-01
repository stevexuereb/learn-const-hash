mod hash_ring;
fn main() {
    let cell_0 = hash_ring::Node::new(1, "cell-00".to_string());

    println!("{:#?}", cell_0.id);
    println!("{:#?}", cell_0.name);
}
