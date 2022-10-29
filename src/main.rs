mod wfc;
use std::time::Instant;

use wfc::{WFC, Frame};
mod tileset;
use tileset::Tileset;

fn main() {
    let tileset :Tileset = Tileset::new("./tilesets/simple.json".to_string());
    let loops = vec![10,15,20,25,30,35,40,50,60,70,80,90,100];
    for (_i, size) in loops.iter().enumerate(){
        let start = Instant::now();
        let mut wave_function_collapse = WFC::new(tileset.clone(), *size as usize);
        wave_function_collapse.resolve();
        // wave_function_collapse.print();
        let duration = start.elapsed();
        println!("Time elapsed in collapsing {} tiles is: {:?}", size*size, duration);
    }
    
    
}

fn _print(tilesheet: Vec<char>, board: Vec<Vec<Frame>>) {
    println!("--------------------");
    for line in board.clone() {
        let v: Vec<char> = line.into_iter().map(|frame| {
                if frame.collapsed {
                    tilesheet[frame.options[0].value as usize]
                } else {
                    ' '
                }
            }).collect();
        let s: String = v.into_iter().collect();
        println!("{}", s);
    }
    println!("--------------------");
}