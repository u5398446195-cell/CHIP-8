mod window;
mod chip;

use window::window;
use crate::chip::Chipe;


fn main(){
let mut chip = Chipe::new();
chip.load_rom("roms/ibm.ch8");
window(&mut chip);
}