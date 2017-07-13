use mgldraw::MGLDraw;
use guy::Guy;
use intface::RenderRage;
use player::{Weapon, player};

static mut rageWpn: u8 = 0;

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
        _ => return
    };
}

#[no_mangle]
pub extern fn UpdateRage(_mgl: *mut MGLDraw) -> u8 {
    1
}

#[no_mangle]
pub unsafe extern fn DoRage(me: &mut Guy) {
    use player::Weapon::*;
    use bullet::*;
    use mgldraw::MGL_random;
    use sound::{self, Sound};
    use options::{PlayAs, opt};

    unsafe fn corner() -> (i32, i32) {
        let (cx, cy) = ::display::get_camera();
        (cx - 320, cy - 240)
    }

    if player.rageClock > 0 {
        player.rageClock -= 1;
    }
    if player.rageClock < 60 {
        return;
    }
    match Weapon::from_int(rageWpn as usize) {
        Some(WPN_NONE) => match PlayAs::from_int(opt.playAs as usize) {
            Some(PlayAs::PLAYAS_BOUAPHA) => if player.rageClock % 4 == 0 {
                HammerLaunch(me.x, me.y, me.facing, 5, (HMR_REVERSE | HMR_REFLECT).bits());
            },
            Some(PlayAs::PLAYAS_LUNATIC) => if player.rageClock % 4 == 0 {
                for _ in 0..10 {
                    fire_bullet(me.x, me.y, MGL_random(8) as u8, Bullet::BLT_BALLLIGHTNING, 1);
                }
            },
            Some(PlayAs::PLAYAS_HAPPY) => if player.rageClock % 4 == 0 {
                HappyLaunch(me.x, me.y, me.facing, 5, (HMR_REVERSE | HMR_REFLECT).bits());
            },
            _ => {}
        },
        Some(WPN_MISSILES) => {
            fire_bullet(me.x, me.y, (player.rageClock & 7), Bullet::BLT_MISSILE, 1);
        }
        Some(WPN_BOMBS) => {
            let (cx, cy) = corner();
            fire_bullet(
                (cx + MGL_random(640)) << ::FIXSHIFT,
                (cy + MGL_random(480)) << ::FIXSHIFT,
                0,
                Bullet::BLT_BOOM,
                1
            );
            ::display::ShakeScreen(10);
        }
        Some(WPN_AK8087) => {
            fire_bullet(me.x, me.y, MGL_random(8) as u8, Bullet::BLT_LASER, 1);
            fire_bullet(me.x, me.y, MGL_random(8) as u8, Bullet::BLT_LASER, 1);
            fire_bullet(me.x, me.y, MGL_random(8) as u8, Bullet::BLT_LASER, 1);
        }
        Some(WPN_FLAME) => {
            let (cx, cy) = corner();
            for _ in 0..3 {
                fire_bullet(
                    (cx + MGL_random(640)) << ::FIXSHIFT,
                    (cy + MGL_random(480)) << ::FIXSHIFT,
                    MGL_random(8) as u8,
                    Bullet::BLT_FLAME,
                    1
                );
            }
        }
        Some(WPN_BIGAXE) => {
            if player.rageClock % 5 == 0 {
                sound::make_sound(Sound::SND_BOMBTHROW, me.x, me.y, sound::SND_CUTOFF, 1200);
                fire_bullet(me.x, me.y, me.facing, Bullet::BLT_BIGAXE, 1);
            }
        }
        Some(WPN_LIGHTNING) => {
            let (cx, cy) = corner();
            fire_bullet(
                (cx + MGL_random(640)) << ::FIXSHIFT,
                (cy + MGL_random(480)) << ::FIXSHIFT,
                MGL_random(8) as u8,
                Bullet::BLT_LIGHTNING,
                1
            );
        }
        Some(WPN_SPEAR) => {
            if player.rageClock % 3 == 0 {
                sound::make_sound(Sound::SND_BOMBTHROW, me.x, me.y, sound::SND_CUTOFF, 1200);
                fire_bullet(me.x, me.y, (me.facing + 7) & 7, Bullet::BLT_SPEAR, 1);
                fire_bullet(me.x, me.y, me.facing, Bullet::BLT_SPEAR, 1);
                fire_bullet(me.x, me.y, (me.facing + 1) & 7, Bullet::BLT_SPEAR, 1);
            }
        }
        Some(WPN_MACHETE) => {
            let (cx, cy) = corner();
            for _ in 0..10 {
                fire_bullet((cx + MGL_random(640)) << ::FIXSHIFT, cy + MGL_random(480) << ::FIXSHIFT,
                    MGL_random(8) as u8, Bullet::BLT_SLASH, 1);
            }
        }
        Some(WPN_MINES) => {
            if player.rageClock % 8 == 0 {
                let r = 32 * (32 / 8 - ((player.rageClock as i32 - 60) / 8) + 1);
                for i in 0..8 {
                    use cossin::{Cosine, Sine};
                    fire_bullet(
                        me.x + Cosine(i * 32) * r,
                        me.y + Sine(i * 32) * r,
                        0,
                        Bullet::BLT_BOOM,
                        1
                    );
                }
            }
        }
        Some(WPN_MINDCONTROL) => {
            if (player.rageClock & 1) != 0 {
                for i in 0..8 {
                    fire_bullet(me.x, me.y, i, Bullet::BLT_MINDWIPE, 1);
                }
            }
        }
        Some(WPN_REFLECTOR) => {
            fire_bullet(me.x, me.y, 0, Bullet::BLT_REFLECT, 1);
        }
        Some(WPN_TURRET) | Some(WPN_SWAPGUN) => {
            for i in 0..4 {
                fire_bullet(me.x, me.y, (i as i32 * 64 + player.rageClock as i32) as u8, Bullet::BLT_GREEN, 1);
            }
        }
        Some(WPN_JETPACK) => {
            for i in 0..8 {
                fire_bullet(me.x, me.y, i, Bullet::BLT_FLAME, 1);
            }
        }
        _ => {}
    }
}
