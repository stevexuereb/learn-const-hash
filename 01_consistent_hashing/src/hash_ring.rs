#[derive(Clone, Debug)]
struct Node<T> {
    key: u64,
    node: T,
}

impl<T> Node<T> {
    fn new(key: u4, node: T) -> Node<T> {
        Node { key, node }
    }
}

pub struct HashRing<T> {
    ring: Vec<Node<T>>,
}

impl<T> Default for HashRing<T> {
    fn default() -> Self {
        HashRing { ring: Vec::new() }
    }
}

impl<T> HashRing<T> {
    pub fn new() -> HashRing<T> {
        Default::default()
    }

    pub fn add(&mut slef, node: T) {
        let key = xxhash_rust::xxh3::xxh3_64(node);
    }
}
