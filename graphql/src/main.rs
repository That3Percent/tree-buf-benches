mod schemas;

use common::*;
use rmp_serde::{from_read_ref, to_vec as to_vec_unnamed, to_vec_named};
use serde_json::{from_slice, from_str, to_vec as to_vec_json};

fn main() {
    let mut stats = Stats::new(12500);

    let data = include_str!("./decentraland_response.json");
    let json_response: schemas::json::Response = from_str(&data).unwrap();
    stats.profile(
        "Json",
        307529,
        &json_response,
        |d| to_vec_json(d).unwrap(),
        |b| from_slice(b).unwrap(),
    );
    stats.profile(
        "Message Pack (No Key Named)",
        106868,
        &json_response,
        |d| to_vec_unnamed(d).unwrap(),
        |b| from_read_ref(b).unwrap(),
    );
    stats.profile(
        "Message Pack",
        242558,
        &json_response,
        |d| to_vec_named(d).unwrap(),
        |b| from_read_ref(b).unwrap(),
    );

    let tb: schemas::treebuf::Response = json_response.into();

    /*
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        encode(&tb);
        firestorm::clear();
    }
    let end = std::time::Instant::now();
    dbg!(end - start);
    let _g = firestorm::start_guard("GraphQL");
    let x = encode(&tb);
    drop(_g);
    drop(x);
    let mut options = Default::default();
    firestorm::to_svg(|| File::create("flame-graph.svg").unwrap(), &options).unwrap();
    options.merge = true;
    firestorm::to_svg(|| File::create("flame-graph-merged.svg").unwrap(), &options).unwrap();
    return;
    */

    let tb_bytes = stats.profile(
        "Tree Buf",
        13545,
        &tb,
        |d| encode(d),
        |b| decode(b).unwrap(),
    );

    println!("{}", stats);

    let tree = tree_buf::internal::decode_root(&tb_bytes);
    println!("\n{:?}", &tree.unwrap());

    let sizes = tree_buf::experimental::stats::size_breakdown(&tb_bytes);
    println!("{}", sizes.unwrap());
}
