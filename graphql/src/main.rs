mod schemas;

use common::*;
use rmp_serde::{from_read_ref, to_vec as to_vec_message_pack};
use serde_json::{from_slice, from_str, to_vec as to_vec_json};
use std::fs::File;

fn main() {
    let mut stats = Stats::new(15000);

    let data = include_str!("./decentraland_response.json");
    let json_response: schemas::json::Response = from_str(&data).unwrap();
    /*
    stats.profile(
        "Json",
        366546,
        &json_response,
        |d| to_vec_json(d).unwrap(),
        |b| from_slice(b).unwrap(),
    );

    stats.profile(
        "Message Pack",
        200728,
        &json_response,
        |d| to_vec_message_pack(d).unwrap(),
        |b| from_read_ref(b).unwrap(),
    );
    */
    let tb: schemas::treebuf::Response = json_response.into();

    /*
    for _ in 0..10 {
        encode(&tb);
    }
    flame::clear();
    let _g = flame::start_guard("GraphQL");
    let x = encode(&tb);
    drop(_g);
    drop(x);
    flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    return;
    */

    let tb_bytes = stats.profile(
        "Tree Buf",
        51471,
        &tb,
        |d| encode(d),
        |b| decode(b).unwrap(),
    );

    println!("{}", stats);

    let tree = tree_buf::internal::decode_root(&tb_bytes);
    println!("\n{:?}", &tree.unwrap());

    let sizes = tree_buf::experimental::stats::size_breakdown(&tb_bytes);
    println!("{}", sizes.unwrap());

    //drop(_g);
    //flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
