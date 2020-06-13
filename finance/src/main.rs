mod model_csv;

use common::*;
use tree_buf::prelude::*;

fn main() {
    //let decoded = inflate::inflate_bytes_zlib(&encoded[..]).unwrap();
    //let as_str = from_utf8(&decoded);

    let data = include_bytes!("data.zip");
    let data = model_csv::decode(&data[..]);
    let mut stats = Stats::new(150);

    stats.profile(
        "Csv",
        1844172,
        &data,
        |r| model_csv::encode(r),
        |b| model_csv::decode(b),
    );
    let bytes = stats.profile("TreeBuf", 0, &data, |r| encode(r), |b| decode(b).unwrap());

    println!("{}", &stats);

    let tree = tree_buf::internal::decode_root(&bytes);
    println!("\n{:?}", &tree.unwrap());
}
