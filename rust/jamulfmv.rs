use flic::*;
use mgldraw::*;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub unsafe extern fn FLI_play(name: *const c_char, loop_: u8, wait: u16, mgl: &mut MGLDraw) {
    let name_str = CStr::from_ptr(name).to_str().unwrap();
    let mut flic = FlicFile::open(name_str.as_ref()).unwrap();
    let width = flic.width() as usize;
    let height = flic.height() as usize;

    let mut buf = vec![0; width * height];
    let mut pal = vec![0; 3 * 256];
    let mut mgl_pal = vec![palette_t { red: 0, green: 0, blue: 0, alpha: 0 }; 256];

    mgl.LastKeyPressed(); // clear key buffer

    // if this is a FLC, skip the first frame
    /*if name_str.ends_with("c") || name_str.ends_with("C") {
        let _ =
        // a confusion issue
    }*/

    let wait_duration = ::std::time::Duration::from_millis(wait as u64);

    loop {
        let info = flic.read_next_frame(&mut RasterMut::new(width, height, &mut buf, &mut pal)).unwrap();

        if info.palette_updated {
            for (dst, src) in mgl_pal.iter_mut().zip(pal.chunks(3)) {
                *dst = palette_t { red: src[0], green: src[1], blue: src[2], alpha: 255 };
            }
            mgl.SetPalette(&mgl_pal);
        }

        let mgl_buf = ::std::slice::from_raw_parts_mut(mgl.GetScreen(), 640 * 480);
        for x in 0..width {
            for y in 0..height {
                let e = buf[width * y + x];
                let basis = 640 * y * 2 + x * 2;
                #[cfg(feature="scale2x")] {
                    let b = if y > 0 { buf[width * (y - 1) + x] } else { e };
                    let d = if x > 0 { buf[width * y + x - 1] } else { e };
                    let h = if y < height - 1 { buf[width * (y + 1) + x] } else { e };
                    let f = if x < width - 1 { buf[width * y + x + 1] } else { e };
                    if b != h && d != f {
                        mgl_buf[basis] = if d == b { d } else { e };
                        mgl_buf[basis + 1] = if b == f { f } else { e };
                        mgl_buf[basis + 640] = if d == h { d } else { e };
                        mgl_buf[basis + 641] = if h == f { f } else { e };
                    } else {
                        mgl_buf[basis] = e;
                        mgl_buf[basis + 1] = e;
                        mgl_buf[basis + 640] = e;
                        mgl_buf[basis + 641] = e;
                    }
                }
                #[cfg(not(feature="scale2x"))] {
                    mgl_buf[basis] = e;
                    mgl_buf[basis + 1] = e;
                    mgl_buf[basis + 640] = e;
                    mgl_buf[basis + 641] = e;
                }
            }
        }

        ::game::HandleCDMusic();
        mgl.Flip();
        if wait > 0 {
            ::std::thread::sleep(wait_duration);
        }
        if (loop_ == 0 && (info.ended || info.looped)) ||
            !mgl.Process() ||
            mgl.LastKeyPressed() == 27 // key #27 is escape
        {
            break
        }
    }
}
