use common::*;
mod geojson;
use flame;
use std::fs::File;

use tree_buf::decode_options;

fn main() {
    //let _g = flame::start_guard("GeoJson");
    let mut stats = Stats::new(150);

    let data = geojson::load_data(&mut stats);
    let bytes = stats.profile("Tree-Buf", 0, &data, |d| encode(d), |b| decode(b).unwrap());
    // TODO: This size is fluctuating by 1 byte - probably because dictionary
    //assert_eq!(6861323, bytes.len());

    /*
    let _all2: geojson::model::GeoJson = time_it("Tree-Buf Serial", &mut stats.decode, || {
        let options = decode_options! { options::DisableParallel };
        tree_buf::read_with_options(&bytes, &options).unwrap()
    });
    */
    //drop(_g);
    //flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    //return;
    let _attributes: geojson::model::GeoJsonAttributes =
        time_it("Tree-Buf Attr", &mut stats.decode, || {
            decode(&bytes).unwrap()
        }, stats.count);

    stats.profile(
        "Tree-Buf L09",
        1840197,
        &data,
        |d| tree_buf::encode_with_options(d, &encode_options! { options::LossyFloatTolerance(-9) }),
        |b| decode(b).unwrap(),
    );

    stats.profile(
        "Tree-Buf L12",
        2264100,
        &data,
        |d| {
            tree_buf::encode_with_options(d, &encode_options! { options::LossyFloatTolerance(-12) })
        },
        |b| decode(b).unwrap(),
    );

    println!("{}", &stats);

    let tree = tree_buf::internal::decode_root(&bytes);
    println!("\n{:?}", &tree.unwrap());

    let sizes = tree_buf::experimental::stats::size_breakdown(&bytes);
    println!("{}", sizes.unwrap());

    //drop(_g);
    //flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
}
