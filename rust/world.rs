use libc::*;
use mgldraw::MGLDraw;
use map::Map;
use std::mem;

pub const MAX_MAPS: usize = 24;

bitflags! {
    /// terrain flags
    pub struct TerrainFlags: u16 {
        const TF_SOLID = 1;
        const TF_ICE = 2;
        const TF_MUD = 4;
        const TF_WATER = 8;
        const TF_LAVA = 16;
        /// if this is the roof of a wall, the wall is pushable
        const TF_PUSHY = 32;
        /// only PUSHON terrain can have things pushed over it
        const TF_PUSHON = 64;
        const TF_ANIM = 128;
        const TF_STEP = 256;
        const TF_DESTRUCT = 512;
        const TF_TRANS = 1024;
        const TF_MINECART = 2048;
        const TF_BUNNY = 4096;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct terrain_t {
    pub flags: u16,
    pub next: u8,
}

#[repr(C)]
pub struct world_t {
    pub numMaps: u8,
    pub totalPoints: c_int,
    pub map: [*mut Map; MAX_MAPS],
    pub terrain: [terrain_t; 200],
}

pub const ZERO_WORLD: world_t = world_t {
    numMaps: 0,
    totalPoints: 0,
    map: [0 as *mut Map; MAX_MAPS],
    terrain: [terrain_t { flags: 0, next: 0 }; 200],
};

#[no_mangle]
pub unsafe extern fn NewWorld(world: &mut world_t, mgl: &mut MGLDraw) -> u8 {
    world.numMaps = 1;
    mgl.LoadBMP(cstr!("graphics\\forestTiles.bmp"));
    ::tile::SetTiles(mgl.GetScreen());

    // reset all the terrain
    world.terrain = mem::zeroed();
    world.map = mem::zeroed();
    world.map[0] = Map::new(0, cstr!("New World"));
    1
}

#[no_mangle]
pub unsafe extern fn LoadWorld(world: &mut world_t, fname: *const c_char) -> u8 {
    use player::player;

    let f = fopen(fname, cstr!("rb"));
    if f.is_null() { return 0; }

    fread(decay!(&mut world.numMaps), 1, 1, f);
    fread(decay!(&mut world.totalPoints), 1, 4, f);

    ::tile::LoadTiles(f);

    fread(decay!(&mut world.terrain), 200, szof!(terrain_t), f);

    for map in world.map.iter_mut() {
        *map = ::std::ptr::null_mut();
    }

    for i in 0..(world.numMaps) {
        world.map[i as usize] = Map::from_file(f);
    }

    player.levelsPassed = 0;
    for i in 0..(world.numMaps) {
        let flags = (*world.map[i as usize]).flags;
        if player.levelPassed[player.worldNum as usize][i as usize] != 0 &&
            !flags.contains(::map::MAP_SECRET)
        {
            player.levelsPassed += 1;
        }
    }

    fclose(f);
    1
}

#[no_mangle]
pub unsafe extern fn SaveWorld(world: &mut world_t, fname: *const c_char) -> u8 {
    world.totalPoints = 0;
    for &map in world.map[1..].iter() {
        if !map.is_null() {
            world.totalPoints += 100; // each level is worth 100 points except the hub which is worth nothing
        }
    }

    let f = fopen(fname, cstr!("wb"));
    if f.is_null() { return 0; }

    fwrite(decay!(&world.numMaps), 1, 1, f);
    fwrite(decay!(&world.totalPoints), 1, szof!(c_int), f);

    ::tile::SaveTiles(f);

    fwrite(decay!(&world.terrain), 200, szof!(terrain_t), f);

    for i in 0..(world.numMaps) {
        (*world.map[i as usize]).Save(f);
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
pub unsafe extern fn InitWorld(world: &mut world_t, worldNum: u8) {
    let mut complete = 0;
    for &map in world.map[1..].iter() {
        if !map.is_null() {
            complete += 100; // each level is worth 100 points except the hub which is worth nothing
        }
    }

    ::player::PlayerSetWorldWorth(worldNum, complete);
    world.totalPoints = complete;
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
    fread(decay!(&mut i), 1, 1, f);
    // read the int totalPoints
    fread(decay!(&mut i), 1, 4, f);
    fclose(f);
    i
}
