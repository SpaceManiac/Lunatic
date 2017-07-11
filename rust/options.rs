cpp! {{
    #include "options.h"
}}

pub unsafe fn sound() -> bool {
    cpp!([] -> bool as "bool" { return opt.sound; })
}
