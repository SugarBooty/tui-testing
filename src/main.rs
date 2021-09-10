use std::{thread, time};
use sysinfo::{SystemExt};


mod get_data;
use get_data::temps::{self, TempMaps};

mod tui_main;
use tui_main::main::{Display};

fn main() {
    let mut s = sysinfo::System::new();
    let mut temp_handler = TempMaps::new(s);
    let mut display_manager = Display::new(&mut temp_handler);
    print!("{esc}c", esc = 27 as char);
    loop {
        display_manager.draw_display();
        // temp_handler.append_temps();
        thread::sleep(time::Duration::from_millis(500));
    }
}