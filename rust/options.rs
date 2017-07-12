use mgldraw::MGLDraw;

#[repr(C)]
pub enum Music {
    MUSIC_OFF = 0,
    MUSIC_ON,
    MUSIC_RAND,
}

#[repr(C)]
#[derive(FromInt)]
pub enum PlayAs {
    PLAYAS_BOUAPHA = 0,
    PLAYAS_LUNATIC,
    PLAYAS_HAPPY,
}

#[repr(C)]
pub struct options_t {
    /// key scancodes
    pub control: [[u8; 6]; 2],
    /// joystick 'codes' for the buttons
    pub joyCtrl: [u8; 2],
    pub sound: u8,
    pub music: u8,
    /// if you wish to play as a different character after winning
    pub playAs: u8,
    pub wonGame: u8,
    pub gotAllSecrets: u8,
    pub youSuck: u8,
    // new options
    pub discoMode: u8,
    pub smoothLight: u8,
}

extern {
    pub static mut opt: options_t;
    pub static mut oldPlayAs: u8;

    pub fn LoadOptions();
    pub fn SaveOptions();
    pub fn OptionsMenu(mgl: *mut MGLDraw);
}
