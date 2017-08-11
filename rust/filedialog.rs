use libc::{c_int, c_char, intptr_t, strncpy};
use mgldraw::MGLDraw;
use ffi::win::*;

const MAX_FILES: usize = 18;

static mut fnames: [[c_char; 32]; MAX_FILES] = [[0; 32]; MAX_FILES];
static mut newfname: [c_char; 32] = [0; 32];
static mut numFiles: u8 = 0;
static mut hFile: intptr_t = 0;

#[no_mangle]
pub unsafe extern fn InitFileDialog() {
    for fname in fnames.iter_mut() {
        fname[0] = 0;
    }
    numFiles = 0;

    let mut filedata: _finddata_t = ::std::mem::uninitialized();
    hFile = _findfirst(cstr!("worlds\\*.dlw"), &mut filedata);
    if hFile != -1 {
        strncpy(fnames[0].as_mut_ptr(), filedata.name.as_ptr(), 32);
        numFiles = 1;
        while numFiles < MAX_FILES as u8 && _findnext(hFile, &mut filedata) == 0 {
            strncpy(fnames[numFiles as usize].as_mut_ptr(), filedata.name.as_ptr(), 32);
            numFiles += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn ExitFileDialog() {
    _findclose(hFile);
}

#[no_mangle]
pub unsafe extern fn RenderFileDialog(msx: c_int, msy: c_int, mgl: &mut MGLDraw) {
    use display::Print;

    // box for the whole dialog
    mgl.FillBox(100, 80, 430, 400, 8);
    mgl.Box(100, 80, 430, 400, 16);

    // the box that contains the file list
    mgl.Box(102, 82, 362, 340, 16);
    mgl.FillBox(103, 83, 361, 339, 0);

    for i in 0..(MAX_FILES as c_int) {
        if msx > 104 && msx < 362 && msy > 85 + i * 14 && msy < 85 + (i + 1)*14 {
            mgl.Box(104, 84 + i * 14, 360, 84 + (i + 1)*14, 16); // hilite if the cursor is on it
        }
    }

    // the box to enter a new filename
    mgl.Box(102, 342, 362, 356, 16);
    mgl.FillBox(103, 343, 361, 355, 0);

    // now the buttons
    mgl.Box(102, 358, 182, 372, 16);
    mgl.Box(370, 92, 420, 92 + 14, 16);
    mgl.Box(370, 180, 420, 180 + 14, 16);
    mgl.Box(370, 270, 420, 270 + 14, 16);
    mgl.Box(370, 370, 420, 370 + 14, 16);

    for i in 0..MAX_FILES {
        Print(107, (86 + i * 14) as i32, fnames[i].as_ptr(), 0, 1);
    }

    Print(104, 344, newfname.as_ptr(), 0, 1);
    Print(104, 360, cstr!("More Files"), 0, 1);
    Print(372,  94, cstr!("New"), 0, 1);
    Print(372, 182, cstr!("Load"), 0, 1);
    Print(372, 272, cstr!("Save"), 0, 1);
    Print(372, 372, cstr!("Quit"), 0, 1);
}

#[no_mangle]
pub unsafe extern fn FileDialogKey(key: c_char) -> u8 {
    if key == 27 { // esc
        0
    } else if key == 8 { // backspace
        let len = ::libc::strlen(newfname.as_ptr());
        if len > 0 {
            newfname[len - 1] = 0;
        }
        1
    } else if key == 10 { // enter
        1 // ignore it- what does enter do?  Load or save?
    } else if ::libc::isprint(key as i32) != 0 {
        let len = ::libc::strlen(newfname.as_ptr());
        if len < 30 {
            newfname[len] = key;
            newfname[len + 1] = 0;
        }
        1
    } else {
        // non-printables keep ending up in the PixelToaster keyboard
        1
    }
}

unsafe fn FileDialogMoreFiles() {
    for fname in fnames.iter_mut() {
        fname[0] = 0;
    }
    numFiles = 0;

    let mut filedata: _finddata_t = ::std::mem::uninitialized();
    while numFiles < MAX_FILES as u8 && _findnext(hFile, &mut filedata) == 0 {
        strncpy(fnames[numFiles as usize].as_mut_ptr(), filedata.name.as_ptr(), 32);
        numFiles += 1;
    }
    if numFiles == 0 {
        ExitFileDialog(); // there aren't any more to list at all!
        InitFileDialog(); // reget the first page of them
    }
}

#[no_mangle]
pub unsafe extern fn FileDialogClick(msx: c_int, msy: c_int) -> u8 {
    use editor::*;

    let mut fname = [0; 64];
    sprintf!(fname, "worlds\\{}", ::PctS(newfname.as_ptr()));
    // if click on a filename, that's the current filename
    for i in 0..MAX_FILES {
        if msx > 104 && msx < 362 &&
            msy > 85 + i as c_int * 14 &&
            msy < 85 + (i as c_int + 1) * 14
        {
            ::libc::strcpy(newfname.as_mut_ptr(), fnames[i].as_ptr());
            return 1;
        }
    }

    // shareware version doesn't let you do this
    if msx > 102 && msx < 182 && msy > 358 && msy < 372 { // More Files
        FileDialogMoreFiles();
        1
    } else if msx > 370 && msy > 92 && msx < 420 && msy < 92 + 14 { // New
        EditorNewWorld();
        0
    } else if msx > 370 && msy > 180 && msx < 420 && msy < 180 + 14 { // Load
        if !fname.starts_with(b"worlds\\backup_load.dlw\0") {
            EditorSaveWorld(cstr!("worlds\\backup_load.dlw"));
        }
        EditorLoadWorld(fname.as_ptr() as *const c_char);
        0
    } else if msx > 370 && msy > 270 && msx < 420 && msy < 270 + 14 { // Save
        EditorSaveWorld(fname.as_ptr() as *const c_char);
        0
    } else if msx > 370 && msy > 370 && msx < 420 && msy < 370 + 14 { // Quit
        0
    } else {
        1
    }
}
