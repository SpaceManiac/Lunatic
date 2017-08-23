use libc::{FILE, c_int, c_char, fread, fwrite};
use mgldraw::MGLDraw;

pub const TILE_WIDTH: c_int = 32;
pub const TILE_HEIGHT: c_int = 24;
pub const NUMTILES: usize = 400;

pub type tile_t = [u8; TILE_WIDTH as usize * TILE_HEIGHT as usize];

extern {
    static mut tiles: [tile_t; NUMTILES];
    static mut tileMGL: *mut MGLDraw;
}

#[no_mangle]
pub unsafe extern fn InitTiles(mgl: *mut MGLDraw) {
    tileMGL = mgl;
}

#[no_mangle]
pub extern fn ExitTiles() {}

pub unsafe fn set_tiles(scrn: &[u8]) {
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
pub unsafe extern fn SetTiles(scrn: *const u8) {
    set_tiles(::std::slice::from_raw_parts(scrn, 640 * 480));
}

#[no_mangle]
pub unsafe extern fn LoadTiles(f: *mut FILE) {
    fread(decay!(&mut tiles), NUMTILES, szof!(tile_t), f);
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

// Disco!

fn PickDiscoColor() -> u8 {
    32 * [1, 3, 4, 5, 6, 7][unsafe {::libc::rand() % 6} as usize]
}

// Rendering for real!

extern {
    fn RenderFloorTile(x: c_int, y: c_int, t: c_int, light: i8);
    fn RenderFloorTileShadow(x: c_int, y: c_int, t: c_int, light: i8);
    fn RenderFloorTileUnlit(x: c_int, y: c_int, t: c_int);
    fn RenderFloorTileTrans(x: c_int, y: c_int, t: c_int, light: i8);
}

// RenderFloorTile
// RenderFloorTileShadow
// RenderFloorTileUnlit
// RenderFloorTileTrans

// Gouraud!

const GB_WID: c_int = TILE_WIDTH / 2;
const GB_HEI: c_int = TILE_HEIGHT / 2;

fn gouraud_box(mgl: &mut MGLDraw, x: c_int, y: c_int, src: &[u8], light: [i8; 4], trans: bool, disco: bool) {
    use FIXAMT;
    use std::cmp::{min, max};

    let screen = mgl.get_screen();
    let mut dst_index = x + y * 640;
    let mut src_index = 0;

    let mut firstLight = light[0] as c_int * FIXAMT;
    let mut lastLight = light[1] as c_int * FIXAMT;
    let dly1 = (light[2] - light[0]) as c_int * FIXAMT / GB_HEI;
    let dly2 = (light[3] - light[1]) as c_int * FIXAMT / GB_HEI;

    let disco = match disco {
        true => Some(PickDiscoColor()),
        false => None,
    };

    for j in 0..GB_HEI {
        let dlx = (lastLight - firstLight) / GB_WID;
        let mut curLight = firstLight;

        if y + j >= 480 {
            return; // all done
        }
        if y + j >= 0 {
            for i in 0..GB_WID {
                let v = src[src_index as usize];
                if x + i >= 0 && x + i < 640 && (v != 0 || !trans) {
                    let light = (v & 31) as i8 + (curLight / FIXAMT) as i8;
                    screen[dst_index as usize] = disco.unwrap_or(v & !31) |
                        min(31, max(0, light)) as u8;
                }
                dst_index += 1;
                src_index += 1;
                curLight += dlx;
            }
        } else {
            dst_index += GB_WID;
            src_index += GB_WID;
        }
        dst_index += 640 - GB_WID;
        src_index += GB_WID;

        firstLight += dly1;
        lastLight += dly2;
    }
}

// 9 light values are passed in, taken directly from adjacent tiles:
//   0  1  2
//   3  4  5
//   6  7  8
// Each is then averaged with #4 to form the 9 points within the current tile.

unsafe fn render_tile(x: c_int, y: c_int, t: c_int, shadow: u8, wall: bool, trans: bool, theLight: &[i8; 9]) {
    use options::opt;

    if x <= -TILE_WIDTH || y <= -TILE_HEIGHT || x >= 640 || y >= 480 {
        return; // no need to render
    }

    if !opt.smoothLight {
        if shadow == 1 || shadow == 2 {
            RenderFloorTileShadow(x, y, t, theLight[4]);
        } else if trans {
            RenderFloorTileTrans(x, y, t, theLight[4]);
        } else {
            RenderFloorTile(x, y, t, theLight[4]);
        }
        return;
    }

    let mut light = *theLight;
    if wall {
        light[6] = (light[6] + light[4] + light[3] + light[7]) / 4;
        light[8] = (light[8] + light[4] + light[7] + light[5]) / 4;
        light[7] = (light[7] + light[4]) / 2;
        light[3] = light[6];
        light[4] = light[7];
        light[5] = light[8];
        light[0] = light[6];
        light[1] = light[7];
        light[2] = light[8];
    } else {
        light[0] = (light[0] + light[4] + light[3] + light[1]) / 4;
        light[2] = (light[2] + light[4] + light[1] + light[5]) / 4;
        light[6] = (light[6] + light[4] + light[3] + light[7]) / 4;
        light[8] = (light[8] + light[4] + light[7] + light[5]) / 4;
        light[1] = (light[1] + light[4]) / 2;
        light[3] = (light[3] + light[4]) / 2;
        light[5] = (light[5] + light[4]) / 2;
        light[7] = (light[7] + light[4]) / 2;
    }

    if shadow == 0 && !opt.discoMode && light.iter().all(|&x| x == 0) {
        if trans {
            RenderFloorTileTrans(x, y, t, 0);
        } else {
            RenderFloorTileUnlit(x, y, t);
        }
        return;
    }

    match shadow {
        1 => { // right side
            light[2] -= 8;
            light[5] -= 8;
            light[8] -= 8;
        }
        2 => { // top-right
            light[2] -= 8;
            light[5] -= 8;
        }
        3 => { // bottom-right
            light[8] -= 8;
        }
        4 => { // bottom side
            light[6] -= 8;
            light[7] -= 8;
            light[8] -= 8;
        }
        5 => { // bottom and bottom-right
            light[5] -= 8;
            light[6] -= 8;
            light[7] -= 8;
            light[8] -= 8;
        }
        6 => { // bottom and right
            light[2] -= 8;
            light[5] -= 8;
            light[6] -= 8;
            light[7] -= 8;
            light[8] -= 8;
        }
        7 => { // bottom-left
            light[6] -= 8;
            light[7] -= 8;
        }
        _ => {}
    }

    let mgl = &mut *tileMGL;
    let tile = &tiles[t as usize];
    gouraud_box(mgl, x, y, tile, [light[0], light[1], light[3], light[4]], trans, opt.discoMode);
    gouraud_box(mgl, x + GB_WID, y, &tile[GB_WID as usize..], [light[1], light[2], light[4], light[5]], trans, opt.discoMode);
    gouraud_box(mgl, x, y + GB_HEI, &tile[(GB_HEI * TILE_WIDTH) as usize..], [light[3], light[4], light[6], light[7]], trans, opt.discoMode);
    gouraud_box(mgl, x + GB_WID, y + GB_HEI, &tile[(GB_WID + GB_HEI * TILE_WIDTH) as usize..], [light[4], light[5], light[7], light[8]], trans, opt.discoMode);
}

#[no_mangle]
pub unsafe extern fn RenderFloorTileFancy(x: c_int, y: c_int, t: c_int, shadow: u8, theLight: &[i8; 9]) {
    render_tile(x, y, t, shadow, false, false, theLight);
}

#[no_mangle]
pub unsafe extern fn RenderWallTileFancy(x: c_int, y: c_int, t: c_int, theLight: &[i8; 9]) {
    render_tile(x, y, t, 0, true, false, theLight);
}

#[no_mangle]
pub unsafe extern fn RenderRoofTileFancy(x: c_int, y: c_int, t: c_int, trans: bool, _: u8, theLight: &[i8; 9]) {
    render_tile(x, y, t, 0, false, trans, theLight);
}
