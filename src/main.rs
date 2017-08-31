//! cbacklight is a utility for setting and reading the backlight on
//! my Chromebook.
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

const BACKLIGHT_PATH: &str = "/sys/class/backlight/intel_backlight";
const BRIGHTNESS: &str = "brightness";
const MAX_BRIGHTNESS: &str = "max_brightness";
const VERSION: &str = "1.0.0";
    
fn usage(code: i32) {
    println!("cbacklight version {}

usage: cbacklight [percent]

	Set or retrieve the Chromebook's backlight.
", VERSION);
    std::process::exit(code);
}

fn backlight_to_percent(current: u16, max: u16) -> u16 {
    return ((current as f32 / max as f32) * 100.0) as u16;
}

fn backlight_from_percent(percent: u16, max: u16) -> u16 {
    let ival: u32 = (max as u32) * (percent as u32);
    return (ival / 100) as u16;
}

fn get_path(file: &str) -> String {
    let buf: PathBuf = [BACKLIGHT_PATH, file].iter().collect();
    return buf.to_str().unwrap().to_owned();
}

fn read_value(path: &str) -> u16 {
    let mut f_backlight = File::open(path).unwrap();
    let mut buf: Vec<u8> = Vec::new();
   
    let nr = f_backlight.read_to_end(&mut buf).unwrap();
    if buf[nr - 1] == 0x0a {
        buf.truncate(nr - 1);
    }
    
    let backlight_as_string = String::from_utf8(buf).unwrap();
    str::parse(&backlight_as_string).unwrap()
}

fn write_value(val: u16) {
    let valstr = format!("{}\n", val);

    let max_backlight_path = get_path(BRIGHTNESS);
    let mut f_backlight = File::create(&max_backlight_path).unwrap();
    f_backlight.write_all(valstr.as_bytes()).unwrap();
}

fn show_brightness() {
    let max_backlight_path = get_path(MAX_BRIGHTNESS);
    let max = read_value(&max_backlight_path);

    let backlight_path = get_path(BRIGHTNESS);
    let current = read_value(&backlight_path);
    
    println!("{}%", backlight_to_percent(current, max));
}

fn set_brightness(percent: String) {
    if percent == "-h".to_string() {
        usage(0);
    }

    let max_backlight_path = get_path(MAX_BRIGHTNESS);
    let max = read_value(&max_backlight_path);
    let current: u16 = backlight_from_percent(str::parse(&percent).unwrap(), max);
    write_value(current);
}

fn main() {
    let count = std::env::args().count();
    match count {
        1 => show_brightness(),
        2 => set_brightness(std::env::args().nth(1).unwrap()),
        _ => usage(1),
    }
    
    std::process::exit(0);
}
