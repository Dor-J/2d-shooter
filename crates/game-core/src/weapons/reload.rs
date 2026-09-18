//! Clip reload start, tick, and interrupt. Interrupting never refills.

/// Start a reload when the magazine is partial and nothing is already cycling.
pub fn start(ammo: u16, magazine: u16, timer: u16, reload_time: u16) -> Option<u16> {
    if timer == 0 && ammo < magazine && reload_time > 0 {
        Some(reload_time)
    } else {
        None
    }
}

/// Count down one tick. True when the magazine should refill this tick.
pub fn tick(timer: &mut u16) -> bool {
    if *timer == 0 {
        return false;
    }
    *timer -= 1;
    *timer == 0
}

/// Switch, drop, throw, or pickup: cancel without restoring ammo.
pub fn interrupt(timer: &mut u16) {
    *timer = 0;
}
