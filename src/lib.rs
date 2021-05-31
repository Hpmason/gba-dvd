#![no_std]
#![feature(isa_attribute)]

pub mod prelude {
    pub use crate::assets::*;

    pub use crate::collision::*;
    pub use crate::sprite::*;
    pub use crate::address::*;
    pub use crate::tiles::*;
    pub use crate::misc::*;
    pub use crate::irq::*;
}

const GBA_WIDTH: i16 = 240;
const GBA_HEIGHT: i16 = 160;

mod assets;

mod collision;
mod sprite;
mod address;
mod tiles;
mod misc;
mod irq;


