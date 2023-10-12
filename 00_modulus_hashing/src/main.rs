use std::collections::BTreeMap;
use xxhash_rust::xxh3::xxh3_64;

fn main() {
    let mut servers: u64 = 3;
    let orgs = vec!["org-00", "org-01", "org-02", "org-03", "org-04", "org-05"];

    let distribution = modulus_hash(servers, orgs.clone());
    println!("{:?}", distribution);

    servers += 1;
    println!("New server is added, it will change all distribution");

    let distribution = modulus_hash(servers, orgs.clone());
    println!("{:?}", distribution);
}

fn modulus_hash(servers: u64, orgs: Vec<&str>) -> BTreeMap<&str, u64> {
    let mut distribution = BTreeMap::new();

    for org in &orgs {
        let picked_server = xxh3_64(org.as_bytes()) % servers;
        distribution.insert(org.to_owned(), picked_server);
    }

    distribution
}
