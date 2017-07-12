use libc::FILE;

extern {
    pub fn SetTiles(scrn: *mut u8);
    pub fn LoadTiles(f: *mut FILE);
    pub fn SaveTiles(f: *mut FILE);
}
