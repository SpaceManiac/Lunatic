use libc::c_int;
use monster::MonsterType;
use map::Map;
use world::world_t;

#[repr(C)]
pub enum Action {
    ACTION_IDLE = 0,
    ACTION_BUSY
}

#[repr(C)]
#[derive(Clone)]
pub struct Guy {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
    pub oldx: c_int,
    pub oldy: c_int,
    pub dx: c_int,
    pub dy: c_int,
    pub dz: c_int,
    pub mapx: u8,
    pub mapy: u8,
    pub facing: u8,

    /// brain variables for AI
    pub mind: u8,
    pub mind1: u8,
    pub mind2: u8,
    pub mind3: u8,

    pub reload: u8,
    pub poison: u8,

    pub ouch: u8,
    pub action: u8,
    pub frmTimer: u16,
    pub frmAdvance: u16,
    pub frm: u8,
    pub seq: u8,
    pub bright: i8,
    pub friendly: u8,

    pub mindControl: u16,
    pub target: *mut Guy,
    pub parent: *mut Guy,
    pub hp: c_int,
    pub type_: MonsterType,
    /// for collision checks
    pub rectx: c_int,
    pub recty: c_int,
    pub rectx2: c_int,
    pub recty2: c_int,
    /// just a copy of the guy's number
    pub ID: u16,
}

impl Guy {
    pub fn new() -> Guy {
        unsafe { ::std::mem::zeroed() }
    }
}

extern {
    pub static mut goodguy: *mut Guy;

    pub fn EditorUpdateGuys(map: &mut Map);
    pub fn UpdateGuys(map: &mut Map, world: *mut world_t);
    pub fn RenderGuys(light: bool);

    static mut guys: *mut *mut Guy;
    static mut maxGuys: c_int;
    static mut nobody: *mut Guy;
}

#[no_mangle]
pub unsafe extern fn InitGuys(max: c_int) {
    maxGuys = max;

    let mut vec = vec![Box::new(Guy::new()); max as usize];
    guys = vec.as_mut_ptr() as *mut *mut Guy;
    ::std::mem::forget(vec);
}

#[no_mangle]
pub unsafe extern fn ExitGuys() {
    let len = maxGuys as usize;
    Vec::from_raw_parts(guys as *mut Box<Guy>, len, len);
}

unsafe fn guy_list<'a>() -> &'a mut [&'a mut Guy] {
    ::std::slice::from_raw_parts_mut(guys as *mut _, maxGuys as usize)
}

unsafe fn DeleteGuy2(g: *mut Guy) {
    (*g).type_ = MonsterType::MONS_NONE;
    for guy in guy_list() {
        if guy.parent == g {
            DeleteGuy2(*guy);
        }
    }
}

#[no_mangle]
pub unsafe extern fn DeleteGuy(x: c_int, y: c_int, type_: MonsterType) {
    for guy in guy_list() {
        if guy.type_ == type_ && guy.x == x && guy.y == y {
            DeleteGuy2(*guy);
        }
    }
}

#[no_mangle]
pub unsafe extern fn GetGuyPos(guy: u16, x: *mut c_int, y: *mut c_int) -> bool {
    if guy == 65535 { return false }
    // guys[guy] should never be null?

    let guy = &guy_list()[guy as usize];
    *x = guy.x;
    *y = guy.y;
    true
}

#[no_mangle]
pub unsafe extern fn MonsterExists(type_: MonsterType) -> bool {
    guy_list().iter().any(|g| g.type_ == type_)
}

#[no_mangle]
pub unsafe extern fn GetGuy(w: u16) -> *mut Guy {
    guy_list()[w as usize]
}

pub unsafe fn HealGoodguy(amt: u8) {
    if goodguy.is_null() { return }

    (*goodguy).hp = ::std::cmp::min(128, (*goodguy).hp + amt as c_int);
}

/// this checks to see if there is any moss on the chosen tile (x,y in tile coords)
#[no_mangle]
pub unsafe extern fn MossCheck(x: c_int, y: c_int) -> bool {
    use monster::MonsterType::*;
    guy_list().iter().any(|g| {
        [MONS_MOSS, MONS_MOSSGRANDE, MONS_MOSS2].contains(&g.type_) &&
        g.hp > 0 &&
        (g.x >> ::FIXSHIFT) / ::tile::TILE_WIDTH == x &&
        (g.y >> ::FIXSHIFT) / ::tile::TILE_HEIGHT == y
    })
}
