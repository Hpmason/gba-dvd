use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tile4bpp(pub [u32; 8]);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tile8bpp(pub [u32; 16]);

pub trait Tile {
    fn new_tile() -> Self;
    fn copy_to_block(self, ind: usize);
}

impl Tile for Tile4bpp {
    fn new_tile() -> Self {
        Tile4bpp::default()
    }
    fn copy_to_block(self, ind: usize) {
        CHAR_DATA_4.index(ind).write(self);
    }
}

impl Tile for Tile8bpp {
    fn new_tile() -> Self {
        Tile8bpp::default()
    }
    fn copy_to_block(self, ind: usize) {
        CHAR_DATA_8.index(ind).write(self);
    }
}
