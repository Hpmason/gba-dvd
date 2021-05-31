#![no_std]
#![no_main]

use gba::{debug, fatal, prelude::*, warning};
use gba_dvd::prelude::*;

#[panic_handler]
#[allow(unused)]
fn panic(info: &core::panic::PanicInfo) -> ! {
  fatal!("{}", info);
}

fn hide_all_objs() {
  for i in 0..128 {
    OAM_ATTR0.index(i).write(ObjAttr0::new().with_double_disabled(true));
  }
}

fn copy_pal() {
  // Load palette into vram
  for (i, color) in DVD_PAL.iter().enumerate() {
    OBJ_PALETTE.index(i).write(Color(*color));  
  }
}

fn copy_tiles() {
  let dvd = DVD_IMG;
  // Load tiles into vram
  let mut tile: Tile4bpp;
  for i in 0..(dvd.len()/8) {
    tile = Tile4bpp::new_tile();
    for j in 0..8 {
      tile.write(j, dvd[i*8 + j])
    }
    // first index is used for all sprites
    CHAR_DATA_4.index(i).write(tile);
  }
}
#[no_mangle]
pub fn main() -> ! {
  const SETTING: DisplayControl = DisplayControl::new()
    .with_display_mode(0)
    .with_display_bg0(true)
    .with_display_obj(true)
    .with_obj_vram_1d(true);
  DISPCNT.write(SETTING);
  const VBLANK_FLAG: InterruptFlags = InterruptFlags::new().with_vblank(true);

  // Set the IRQ handler to use.
  unsafe { USER_IRQ_HANDLER.write(Some(irq_handler_a32)) };

  // Enable all interrupts that are set in the IE register.
  unsafe { IME.write(true) };

  // Enable vblank irq
  const DISPLAY_SETTINGS: DisplayStatus = DisplayStatus::new()
    .with_vblank_irq_enabled(true);
  DISPSTAT.write(DISPLAY_SETTINGS);

  BG_PALETTE.index(0).write(Color::from_rgb(3, 3, 3));
  hide_all_objs();
  copy_pal();
  copy_tiles();
  // Size should be 64x48 (from top left), but takes up a full 64x64 sprite
  let mut x_dir = Direction::RIGHT;
  let mut y_dir = Direction::UP;
  let window = BoundingBox::new().with_size(240, 160);
  let mut dvd_obj = Obj::new(40, 30, 64, 48);
  dvd_obj.set_size(&SpriteDimensions::Size64x64);
  dvd_obj.set_tile_ind(0);
  dvd_obj.set_pal_ind(0);

  dvd_obj.copy_to_block(0);
  
  // corner counter will count down to zero each frame 
  //    and will be reset if the dvd hits a corner
  let mut corner_counter = 0;
  let mut rng = gba::random::RNG::default();
  loop {
    // Do logic
    // dvd_obj.move_pos(x_dir.value() as i16, y_dir.value() as i16);
    dvd_obj.move_pos(x_dir.value(), y_dir.value());

    let collisions = dvd_obj.bounding_box.is_outside(window);
    if collisions.num_collisions() >= 1 {
      debug!("Position: {} {}", dvd_obj.x(), dvd_obj.y());
      if collisions.top() || collisions.bottom() {
        y_dir = y_dir.flip(); 
      }
      if collisions.left() || collisions.right() {
        x_dir = x_dir.flip();
      }

      OBJ_PALETTE.index(1).write(rng.next_color());
      // Corner hit
      if collisions.num_collisions() == 2 {
        warning!("Corner Hit!");
        corner_counter = 80;
      }
    }
    // If corner bounce color change animation
    if corner_counter > 0 {
      // randomize every few frame
      if corner_counter % 8 == 0 {  
        OBJ_PALETTE.index(1).write(rng.next_color());
      }
      // Decrease counter
      corner_counter -= 1;
    }
    // now we wait
    // spin_until_vblank();
    
    // Draw things
    unsafe { IE.write(VBLANK_FLAG) };

    // now we wait again
    // spin_until_vdraw();
    unsafe { VBlankIntrWait(); }
    dvd_obj.copy_to_block(0);
  }
}