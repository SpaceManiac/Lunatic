use libc::{FILE, c_int, c_char, fread, fwrite};
use mgldraw::MGLDraw;

pub const TILE_WIDTH: c_int = 32;
pub const TILE_HEIGHT: c_int = 24;
pub const NUMTILES: usize = 400;

pub type tile_t = [u8; TILE_WIDTH as usize * TILE_HEIGHT as usize];

extern {
    static mut tiles: [tile_t; NUMTILES];
    static mut tileMGL: *mut MGLDraw;

    pub fn RenderWallTileFancy(x: c_int, y: c_int, t: c_int, theLight: *mut c_char);
    pub fn RenderRoofTileFancy(x: c_int, y: c_int, t: c_int, trans: bool, wallBelow: u8, theLight: *mut c_char);
}

#[no_mangle]
pub unsafe extern fn InitTiles(mgl: *mut MGLDraw) {
    tileMGL = mgl;
}

#[no_mangle]
pub extern fn ExitTiles() {}

#[no_mangle]
pub unsafe extern fn SetTiles(scrn: *const u8) {
    let scrn = ::std::slice::from_raw_parts(scrn, 640 * 480);
    let (mut x, mut y) = (0, 0);
    let (w, h) = (TILE_WIDTH as usize, TILE_HEIGHT as usize);
    for i in 0..NUMTILES {
        for j in 0..h {
            let ofs = x + (y + j) * 640;
            tiles[i][j * w .. (j+1) * w].copy_from_slice(&scrn[ofs..ofs + w])
        }
        x += w;
        if x >= 640 {
            x = 0;
            y += h;
        }
    }
}

#[no_mangle]
pub unsafe extern fn LoadTiles(f: *mut FILE) {
    fread(decay!(&tiles), NUMTILES, szof!(tile_t), f);
}

#[no_mangle]
pub unsafe extern fn SaveTiles(f: *mut FILE) {
    fwrite(decay!(&tiles), NUMTILES, szof!(tile_t), f);
}

#[no_mangle]
pub unsafe extern fn PlotStar(x: c_int, y: c_int, col: u8, tx: u8, ty: u8, tileNum: u8) {
    if tiles[tileNum as usize][tx as usize + ty as usize * TILE_WIDTH as usize] == 0 {
        (*tileMGL).get_screen()[(x + y * 640) as usize] = col;
    }
}
