// build.rs
use gba_image_macros::prelude::*;

fn main() {
    // Create a ConstWriter for the macros to use
    let consts = ConstWriter::from_path(&Path::new("src/assets.rs")).unwrap();
    let mut consts = consts.finish_dependencies();
    // add a load_sprite macro for each sprite
    load_sprite!(consts, "dvd", "DVD");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets");
}
