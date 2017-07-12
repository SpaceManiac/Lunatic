use libc::*;
use mgldraw::MGLDraw;
use map::Map;
use std::mem;

pub const MAX_MAPS: usize = 24;

/// terrain flags
pub mod Terrain {
    pub const TF_SOLID: u16 = 1;
    pub const TF_ICE: u16 = 2;
    pub const TF_MUD: u16 = 4;
    pub const TF_WATER: u16 = 8;
    pub const TF_LAVA: u16 = 16;
    /// if this is the roof of a wall, the wall is pushable
    pub const TF_PUSHY: u16 = 32;
    /// only PUSHON terrain can have things pushed over it
    pub const TF_PUSHON: u16 = 64;
    pub const TF_ANIM: u16 = 128;
    pub const TF_STEP: u16 = 256;
    pub const TF_DESTRUCT: u16 = 512;
    pub const TF_TRANS: u16 = 1024;
    pub const TF_MINECART: u16 = 2048;
    pub const TF_BUNNY: u16 = 4096;
}

#[repr(C)]
pub struct terrain_t {
    pub flags: u16,
    pub next: u8,
}

#[repr(C)]
pub struct world_t {
    numMaps: u8,
    totalPoints: c_int,
    map: [*mut Map; MAX_MAPS],
    terrain: [terrain_t; 200],
}

extern {
    pub fn WorldLoadBMP(name: *mut c_char, dst: *mut u8);
}

#[no_mangle]
pub unsafe extern fn NewWorld(world: *mut world_t, mut mgl: *mut MGLDraw) -> u8 {
    (*world).numMaps = 1;
    cpp!([mut mgl as "MGLDraw*"] {
        mgl->LoadBMP("graphics\\forestTiles.bmp");
        SetTiles(mgl->GetScreen());
    });

    // reset all the terrain
    (*world).terrain = mem::zeroed();
    (*world).map = mem::zeroed();
    (*world).map[0] = Map::new(0, cstr!("New World"));
    1
}

#[no_mangle]
pub unsafe extern fn LoadWorld(world: *mut world_t, fname: *const c_char) -> u8 {
    use player::player;

    let f = fopen(fname, cstr!("rb"));
    if f.is_null() { return 0; }

    fread((&mut (*world).numMaps) as *mut _ as *mut _, 1, 1, f);
    fread((&mut (*world).totalPoints) as *mut _ as *mut _, 1, 4, f);

    ::tile::LoadTiles(f);

    fread((&mut (*world).terrain) as *mut _ as *mut _, 200, szof!(terrain_t), f);

    for map in (*world).map.iter_mut() {
        *map = ::std::ptr::null_mut();
    }

    for i in 0..((*world).numMaps) {
        (*world).map[i as usize] = Map::from_file(f);
    }

    player.levelsPassed = 0;
    for i in 0..((*world).numMaps) {
        let flags = *Map::flags((*world).map[i as usize]);
        if player.levelPassed[player.worldNum as usize][i as usize] != 0 &&
            (flags & ::map::MapFlags::MAP_SECRET) == 0
        {
            player.levelsPassed += 1;
        }
    }

    fclose(f);
    1
}

#[no_mangle]
pub unsafe extern fn SaveWorld(world: *mut world_t, fname: *const c_char) -> u8 {
    (*world).totalPoints = 0;
    for &map in (*world).map[1..].iter() {
        if !map.is_null() {
            (*world).totalPoints += 100; // each level is worth 100 points except the hub which is worth nothing
        }
    }

    let f = fopen(fname, cstr!("wb"));
    if f.is_null() { return 0; }

    fwrite((&(*world).numMaps) as *const _ as *const _, 1, 1, f);
    fwrite((&(*world).totalPoints) as *const _ as *const _, 1, szof!(c_int), f);

    ::tile::SaveTiles(f);

    fwrite((&(*world).terrain) as *const _ as *const _, 200, szof!(terrain_t), f);

    for i in 0..((*world).numMaps) {
        (*(*world).map[i as usize]).Save(f);
    }

    fclose(f);
    1
}

#[no_mangle]
pub unsafe extern fn FreeWorld(world: *mut world_t) {
    for &map in (*world).map.iter() {
        if !map.is_null() {
            Map::delete(map);
        }
    }
}

#[no_mangle]
pub unsafe extern fn InitWorld(world: *mut world_t, worldNum: u8) {
    let mut complete = 0;
    for &map in (*world).map[1..].iter() {
        if !map.is_null() {
            complete += 100; // each level is worth 100 points except the hub which is worth nothing
        }
    }

    ::player::PlayerSetWorldWorth(worldNum, complete);
    (*world).totalPoints = complete;
}

#[no_mangle]
pub unsafe extern fn GetWorldName(fname: *const c_char, buf: *mut c_char) {
    if *fname == 0 { return; }

    let mut fname2 = [0; 60];
    sprintf!(fname2, "worlds\\{}", ::PctS(fname));
    let f = fopen(fname2.as_ptr() as *const c_char, cstr!("rb"));
    if f.is_null() { return; }

	// this fseeks past:
	//   the byte nummaps, the int totalpoints, the 400 32x24 tiles,
	//   the 200 terrain types, the width&height of map 0, and bam there it is at the name
	//   of map 0.
    let ofs = 1 + szof!(c_int) + 400 * 32 * 24 + 200 * szof!(terrain_t) + 2 * szof!(c_int);
    fseek(f, ofs as c_int, SEEK_SET);

    // read the name
    fread(buf as *mut c_void, 1, 32, f);
    fclose(f);
}

#[no_mangle]
pub unsafe extern fn GetWorldPoints(fname: *const c_char) -> c_int {
    if *fname == 0 { return 100; }

    let mut fname2 = [0; 60];
    sprintf!(fname2, "worlds\\{}", ::PctS(fname));
    let f = fopen(fname2.as_ptr() as *const c_char, cstr!("rb"));
    if f.is_null() { return 100; }

    let mut i = mem::uninitialized();
    // skip over the byte
    fread((&mut i) as *mut c_int as *mut c_void, 1, 1, f);
    // read the int totalPoints
    fread((&mut i) as *mut c_int as *mut c_void, 1, 4, f);
    fclose(f);
    i
}
