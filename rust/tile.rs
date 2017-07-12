use libc::FILE;

extern {
    pub fn LoadTiles(f: *mut FILE);
    pub fn SaveTiles(f: *mut FILE);
}
