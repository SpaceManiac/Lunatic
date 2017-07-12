use mgldraw::MGLDraw;
use guy::Guy;
use intface::RenderRage;
use player::{Weapon, player};

extern {
    static mut rageWpn: u8;

    pub fn DoRage(me: *mut Guy);
}

#[no_mangle]
pub unsafe extern fn ShowRage(mgl: *mut MGLDraw) {
    if player.rageClock > 10 {
        RenderRage(15 - player.rageClock, mgl);
    } else if player.rageClock < 6 {
        if player.rageClock > 0 {
            RenderRage(player.rageClock - 1, mgl);
        }
    } else {
        RenderRage(4, mgl);
    }
}

#[no_mangle]
pub unsafe extern fn StartRaging() {
    use player::Weapon::*;

    rageWpn = player.weapon;
    player.rageClock = 60 + match Weapon::from_int(rageWpn as usize) {
        Some(WPN_NONE) => 10,
        Some(WPN_MISSILES) => 16,
        Some(WPN_BOMBS) => 20,
        Some(WPN_AK8087) => 40,
        Some(WPN_FLAME) => 30,
        Some(WPN_BIGAXE) => 50,
        Some(WPN_LIGHTNING) => 40,
        Some(WPN_SPEAR) => 20,
        Some(WPN_MACHETE) => 10,
        Some(WPN_MINES) => 33,
        Some(WPN_MINDCONTROL) => 2,
        Some(WPN_REFLECTOR) => 60,
        Some(WPN_TURRET) => 64,
        Some(WPN_JETPACK) => 12,
        Some(WPN_SWAPGUN) => 32,
        _ => unreachable!()
    };
}

#[no_mangle]
pub extern fn UpdateRage(_mgl: *mut MGLDraw) -> u8 {
    1
}
