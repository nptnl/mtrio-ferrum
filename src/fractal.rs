use std::thread;
use std::{fs::File, sync::atomic::AtomicUsize};
use std::io::prelude::*;
use std::path::Path;
use crate::ch::Comp;
use crate::trig;
use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering::{Relaxed, Release};

static ARR_HEX: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f',];

const CORES: usize = 4;

pub fn quadra(c: Comp, size: i32, iterate: usize) {
    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).unwrap();
    for imag in -size..size {
        let mut colorstring = String::from("");
        for real in -size..size {
            let mut z: Comp = Comp::new(2.0 * real as f32 / size as f32, 2.0 * imag as f32 / size as f32);
            let mut counter = 0;
            while z.r*z.r + z.i*z.i < 4.0 {
                z = z.square() + c;
                counter += 1;
                if counter == iterate { counter = 0; break }
            }
            colorstring = format!("{colorstring}{}", ARR_HEX[counter * 16 / iterate]);
        }
        colorstring += "\n";
        file.write_all(colorstring.as_bytes()).unwrap();
    }
}
pub fn mandelbrot(size: i32, iterate: usize) {
    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).unwrap();
    let count = Arc::new(AtomicUsize::new(0));
    let rows: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![String::new(); (size * 2) as usize]));
    for imag in -size..size {
        let rows = rows.clone();
        let count = count.clone();
        thread::spawn(move || {
            let mut colorstring = String::from("");
            for real in -size..size {
                let c: Comp = Comp::new(2.0 * real as f32 / size as f32, 2.0 * imag as f32 / size as f32);
                let mut z: Comp = Comp::new(0.0,0.0);
                let mut counter = 0;
                while z.r*z.r + z.i*z.i < 4.0 {
                    z = z.square() + c;
                    counter += 1;
                    if counter == iterate { counter = 0; break }
                }
                colorstring = format!("{colorstring}{}", ARR_HEX[counter * 16 / iterate]);
            }
            rows.lock().unwrap()[(imag + size) as usize] = colorstring;
            let current = count.load(Relaxed);
            count.swap(current + 1, Release);
        });
    }
    while count.load(Relaxed) < (size * 2) as usize {}
    file.write_all(rows.lock().unwrap().join("\n").as_bytes()).unwrap();
}

pub fn ispace(c: Comp, size: i32, iterate: usize) {
    let formula = |z: Comp| trig::tan(z) - z.square() + c; // enter your own closure here
    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).unwrap();
    for imag in -size..size {
        let mut colorstring = String::from("");
        for real in -size..size {
            let mut z: Comp = Comp::new(2.0 * real as f32 / size as f32, 2.0 * imag as f32 / size as f32);
            let mut counter = 0;
            while z.r*z.r + z.i*z.i < 4.0 {
                z = formula(z);
                counter += 1;
                if counter == iterate { counter = 0; break }
            }
            colorstring = format!("{colorstring}{}", ARR_HEX[counter * 16 / iterate]);
        }
        colorstring += "\n";
        file.write_all(colorstring.as_bytes()).unwrap();
    }
}
pub fn pspace(size: i32, iterate: usize) {
    let path = Path::new("./plots/current.npxl");
    let mut file = File::create(&path).unwrap();
    let first = format!("{} {}\n", size*2, size*2) + "16 1\n";
    file.write_all(first.as_bytes()).unwrap();
    for imag in -size..size {
        let mut colorstring = String::from("");
        for real in -size..size {
            let c: Comp = Comp::new(2.0 * real as f32 / size as f32, 2.0 * imag as f32 / size as f32);
            let formula = |z: Comp| trig::tan(z) - z.square() + c; // enter your own closure here
            let mut z: Comp = Comp::new(0.0,0.0);
            let mut counter = 0;
            while z.r*z.r + z.i*z.i < 4.0 {
                z = formula(z);
                counter += 1;
                if counter == iterate { counter = 0; break }
            }
            colorstring = format!("{colorstring}{}", ARR_HEX[counter * 16 / iterate]);
        }
        colorstring += "\n";
        file.write_all(colorstring.as_bytes()).unwrap();
    }
}
