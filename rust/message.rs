use libc::{c_char, c_int};
extern {
    pub fn NewMessage(txt: *const c_char, time: c_int, priority: u8);
}
