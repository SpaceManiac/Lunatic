use libc::{c_char, c_int};

#[repr(C)]
#[derive(Default)]
pub struct message_t {
    x: c_int,
    y: c_int,
    dy: c_int,
    timer: c_int,
    bright: c_int,
    brightDir: c_char,
    msg: [c_char; 32],
    priority: u8,
}

static mut bigMessage: message_t = message_t {
    x: 0,
    y: 0,
    dy: 0,
    timer: 0,
    bright: 0,
    brightDir: 0,
    msg: [0; 32],
    priority: 0,
};
static mut message: message_t = message_t {
    x: 0,
    y: 0,
    dy: 0,
    timer: 0,
    bright: 0,
    brightDir: 0,
    msg: [0; 32],
    priority: 0,
};

pub unsafe fn InitMessage() {
    message.msg[0] = 0;
}

pub unsafe fn NewBigMessage(txt: *const c_char, time: c_int) {
    ::libc::strncpy(bigMessage.msg.as_mut_ptr(), txt, 32);
    bigMessage.x = 320 - ::display::GetStrLength(bigMessage.msg.as_ptr()) / 2;
    bigMessage.y = -100;
    bigMessage.dy = 0;
    bigMessage.timer = time;
    bigMessage.bright = -32;
    bigMessage.brightDir = 2;
}

#[no_mangle]
pub unsafe extern fn NewMessage(txt: *const c_char, time: c_int, priority: u8) {
    if message.priority == 1 && priority == 0 {
        return; // can't override it
    }
    ::libc::strncpy(message.msg.as_mut_ptr(), txt, 32);
    message.x = 2;
    message.y = 484;
    message.dy = -13;
    message.timer = time;
    message.bright = -32;
    message.brightDir = 2;
    message.priority = priority;
}

pub unsafe fn UpdateMessage() {
    UpdateBigMessage();

    message.y += message.dy;
    message.dy += 1;
    message.bright += message.brightDir as c_int;

    if message.timer > 0 {
        message.timer -= 1;
    } else {
        message.brightDir = -2;
    }

    // while time still remains, don't start falling offscreen
    if message.timer > 0 {
        if message.dy > 0 {
            message.dy = 0;
        }

        if message.bright >= 32 {
            message.brightDir = -2;
        }
        if message.brightDir < 0 && message.bright < 0 {
            message.brightDir = 0;
            message.bright = 0;
        }
    } else { // go ahead and fall
        if message.y > 480 {
            message.msg[0] = 0;
            message.y = 0;
            message.dy = 0;
            message.priority = 0;
        }
    }
}

unsafe fn UpdateBigMessage() {
    bigMessage.y += bigMessage.dy;
    bigMessage.dy += 2;
    bigMessage.bright += bigMessage.brightDir as c_int;

    if bigMessage.timer > 0 {
        bigMessage.timer -= 1;
    } else {
        bigMessage.brightDir = -2;
    }

    // while time still remains, don't start falling offscreen
    if bigMessage.timer > 0 {
        if bigMessage.y > 200 {
            bigMessage.y = 200;
            bigMessage.dy = -bigMessage.dy / 2;
            if bigMessage.dy > -2 {
                bigMessage.dy = 0;
            }
        }
        if bigMessage.bright >= 32 {
            bigMessage.brightDir = -1;
        }

        if bigMessage.brightDir < 0 && bigMessage.bright < 0 {
            bigMessage.brightDir = 0;
            bigMessage.bright = 0;
        }
    } else { // go ahead and fall
        if bigMessage.y > 480 {
            bigMessage.msg[0] = 0;
            bigMessage.y = 0;
            bigMessage.dy = 0;
        }
    }
}

#[no_mangle]
pub unsafe extern fn NoRepeatNewMessage(txt: *const c_char, time: c_int, priority: u8) {
    if message.priority == 1 && priority == 0 {
        return; // can't override it
    }
    if ::libc::strncmp(message.msg.as_ptr(), txt, 32) == 0 {
        return; // don't reset if showing the same message
    }
    NewMessage(txt, time, priority);
    ::sound::make_normal_sound(::sound::Sound::SND_MESSAGE);
}

pub unsafe fn RenderMessage() {
    use display::Print;
    Print(message.x, message.y, message.msg.as_ptr(), (message.bright / 2) as i8, 0);
    Print(bigMessage.x, bigMessage.y, bigMessage.msg.as_ptr(), (bigMessage.bright / 2) as i8, 0);
}
