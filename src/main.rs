mod wfc;
use wfc::{WFC, Frame};
mod tileset;
use tileset::Tileset;
fn main() {
    let tilesheet = vec!['O', '⊢', '⊣','⊤', '⊥'];
    let tileset :Tileset = Tileset::new("./tilesets/simple.json".to_string());
    let size = 30; 
    let mut wave_function_collapse = WFC::new(tileset, size);
    print(tilesheet, wave_function_collapse.resolve());
    
}

fn print(tilesheet: Vec<char>, board: Vec<Vec<Frame>>) {
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