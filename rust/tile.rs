use libc::{FILE, c_int};

pub const TILE_WIDTH: c_int = 32;
pub const TILE_HEIGHT: c_int = 24;
pub const NUMTILES: c_int = 400;

pub type tile_t = [u8; TILE_WIDTH as usize * TILE_HEIGHT as usize];

extern {
    pub fn SetTiles(scrn: *mut u8);
    pub fn LoadTiles(f: *mut FILE);
    pub fn SaveTiles(f: *mut FILE);
}
