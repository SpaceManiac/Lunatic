# init.tcl
# defines public-facing procedures for LoonyMod's Tcl scripting

########################################
# items
array set itemNames {
    hammer 1        pants 2
    reverse 3       reflect 4
    missiles 5      ak8087 6
    chinese 7       shield 8
    bombs 9         flame 10
    brain 11        keych1 12
    keych2 13       keych3 14
    keych4 15       keyY 16
    keyR 17         keyG 18
    keyB 19         loony 20
    axe 21          armor 22
    zapwand 23      spear 24
    machete 25      mines 26
    garlic 27       orbiter 28
    accelerator 29
}

proc giveItem {id {pos {0 0}}} {
    global itemNames
    if {[info exists itemNames($id)]} {
        set id $itemNames($id)
    }
    lm_::PlayerGetItem $id [lindex $pos 0] [lindex $pos 1]
}

########################################
# misc
proc repeat {x body} {
    for {set i 0} {$i < $x} {incr i} {
        uplevel 1 $body
    }
}
