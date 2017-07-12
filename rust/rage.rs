use mgldraw::MGLDraw;
use guy::Guy;

extern {
    pub fn ShowRage(mgl: *mut MGLDraw);
    pub fn UpdateRage(mgl: *mut MGLDraw) -> u8;
    pub fn StartRaging();
    pub fn DoRage(me: *mut Guy);
}
