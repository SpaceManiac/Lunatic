use libc::{c_int, c_char};
use world::{world_t, MAX_MAPS};
use mgldraw::MGLDraw;

const MAX_MAPNAMES: usize = MAX_MAPS;

static mut mapnames: [[u8; 40]; MAX_MAPNAMES] = [[0; 40]; MAX_MAPNAMES];
static mut newmapname: [c_char; 32] = [0; 32];
static mut mapNum: u8 = 0;
static mut world_: *mut world_t = 0 as *mut world_t;

unsafe fn reset_mapnames() {
    for name in mapnames.iter_mut() {
        name[0] = 0;
    }

    let world = &mut *world_;
    for (i, (dest, src)) in mapnames.iter_mut().zip(&world.map[..world.numMaps as usize]).enumerate() {
        sprintf!(dest, "{:02}: {}", i, ::PctS((**src).name.as_ptr()));
    }
}

#[no_mangle]
pub unsafe extern fn InitMapDialog(wrld: *mut world_t, currentMap: u8) {
    world_ = wrld;
    mapNum = currentMap;
    reset_mapnames();
}

#[no_mangle]
pub extern fn ExitMapDialog() {}

#[no_mangle]
pub unsafe extern fn RenderMapDialog(msx: c_int, msy: c_int, mgl: &mut MGLDraw) {
    use display::Print;

    // box for the whole dialog
    mgl.FillBox(60, 10, 480, 370, 8);
    mgl.Box(60, 10, 480, 370, 16);
    // the box that contains the map list
    mgl.Box(62, 12, 362, 352, 16);
    mgl.FillBox(63, 13, 361, 351, 0);
    for (i, mapname) in mapnames.iter().enumerate() {
        let i = i as i32;
        Print(67, 16 + i * 14, decay!(mapname.as_ptr()), 0, 1);
        if msx > 64 && msx < 362 && msy > 13 + i * 14 && msy < 14 + (i + 1)*14 {
            mgl.Box(64, 14 + i * 14, 360, 14 + (i + 1)*14, 16); // hilite if the cursor is on it
        }
        if i == mapNum as i32 {
            mgl.Box(64, 14 + i * 14, 360, 14 + (i + 1)*14, 31); // megahilite if this is the selected one
        }
    }
    // the box to enter a new name
    mgl.Box(62, 354, 362, 368, 16);
    mgl.FillBox(63, 355, 361, 367, 0);
    Print(64, 356, newmapname.as_ptr(), 0, 1);

    // now the buttons
    mgl.Box(380, 12, 470, 12 + 14, 16);
    Print(382, 14, cstr!("New Small"), 0, 1);
    mgl.Box(380, 28, 470, 28 + 14, 16);
    Print(382, 30, cstr!("New Medium"), 0, 1);
    mgl.Box(380, 44, 470, 44 + 14, 16);
    Print(382, 46, cstr!("New Large"), 0, 1);
    mgl.Box(380, 60, 470, 60 + 14, 16);
    Print(382, 62, cstr!("Copy"), 0, 1);

    mgl.Box(380, 120, 470, 120 + 14, 16);
    Print(382, 122, cstr!("Edit"), 0, 1);
    mgl.Box(380, 200, 470, 200 + 14, 16);
    Print(382, 202, cstr!("Rename"), 0, 1);
    mgl.Box(380, 280, 470, 280 + 14, 16);
    Print(382, 282, cstr!("Delete"), 0, 1);
}

#[no_mangle]
pub unsafe extern fn MapDialogKey(key: c_char) -> u8 {
    use libc::{strlen, isprint};

    if key == 27 { // esc
        return 0;
    }

    if key == 10 { // enter
        return 1; // ignore it- what does enter do?  Load or save?
    }

    let len = strlen(newmapname.as_ptr());
    if key == 8 { // backspace
        if len > 0 {
            newmapname[len as usize - 1] = 0;
        }
        return 1;
    }

    if isprint(key as c_int) == 0 {
        // non-printables keep ending up in the PixelToaster keyboard
        return 1;
    }

    if len < 31 {
        newmapname[len as usize] = key;
        newmapname[len as usize + 1] = 0;
    }
    return 1;
}

#[no_mangle]
pub unsafe extern fn MapDialogClick(msx: c_int, msy: c_int) -> u8 {
    use map::Map;

    let world = &mut *world_;

    // if click on a mapname, that's the current map
    for i in 0..(MAX_MAPNAMES as u8) {
        if msx > 64 && msx < 362 && msy > 13 + (i as c_int) * 14 && msy < 14 + (i as c_int + 1) * 14 {
            if i < world.numMaps {
                ::editor::EditorSelectMap(i);
                mapNum = i;
            }
        }
    }

    // now the buttons
    if world.numMaps < MAX_MAPS as u8 {
        // new small
        if msx > 379 && msy > 11 && msx < 471 && msy < 11 + 15 {
            world.map[world.numMaps as usize] = Map::new(0, newmapname.as_ptr());
            world.numMaps += 1;
        }
        // new medium
        if msx > 379 && msy > 28 && msx < 471 && msy < 28 + 15 {
            world.map[world.numMaps as usize] = Map::new(1, newmapname.as_ptr());
            world.numMaps += 1;
        }
        // new large
        if msx > 379 && msy > 44 && msx < 471 && msy < 44 + 15 {
            world.map[world.numMaps as usize] = Map::new(2, newmapname.as_ptr());
            world.numMaps += 1;
        }
        // copy
        if msx > 379 && msy > 60 && msx < 471 && msy < 60 + 15 {
            world.map[world.numMaps as usize] = Map::from_map(world.map[mapNum as usize]);
            world.numMaps += 1;
        }
    }

    if msx > 379 && msy > 120 && msx < 471 && msy < 120 + 15 {
        return 0; // edit
    }

    if msx > 379 && msy > 200 && msx < 471 && msy < 200 + 15 {
        // rename
        (*world.map[mapNum as usize]).name = newmapname;
    }

    if msx > 379 && msy > 280 && msx < 471 && msy < 280 + 15 {
        // delete
        if mapNum != 0 { // can't delete the original one
            Map::delete(world.map[mapNum as usize]);
            for i in (mapNum as usize + 1) .. (world.numMaps as usize) {
                world.map[i - 1] = world.map[i];
            }
            world.numMaps -= 1;
            world.map[world.numMaps as usize] = ::std::ptr::null_mut();
            ::editor::EditorSelectMap(0);
            mapNum = 0;
        }
    }

    // reset the mapname stuff in case it was tweaked around, like a map was
    // renamed or deleted
    reset_mapnames();
    1
}
