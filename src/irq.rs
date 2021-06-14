

use gba::prelude::*;
use gba_irq::prelude::*;
pub struct Handler;
impl IRQHandler for Handler {
  const INTERRUPTS_TO_HANDLE: InterruptFlags = InterruptFlags::new()
    .with_vblank(true);
  
}