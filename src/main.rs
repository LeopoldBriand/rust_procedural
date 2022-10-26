mod wfc;
use wfc::{Tile, Rule, WFC};
fn main() {

    let mut tiles: Vec<Tile> = Vec::new();

    // Setup 'BLANK' Tile
    tiles.push(Tile { weight: 10, id: 0, value: 'O', rules: vec![
        Rule{direction: 0, border_type: 0},
        Rule{direction: 1, border_type: 0},
        Rule{direction: 2, border_type: 0},
        Rule{direction: 3, border_type: 0},
    ] });
    // Setup 'EAST' Tile
    tiles.push(Tile { weight: 1, id: 1, value: '⊢', rules: vec![
        Rule{direction: 0, border_type: 1},
        Rule{direction: 1, border_type: 1},
        Rule{direction: 2, border_type: 0},
        Rule{direction: 3, border_type: 1},
    ] });
    // Setup 'WEST' Tile
    tiles.push(Tile { weight: 1, id: 2, value: '⊣', rules: vec![
        Rule{direction: 0, border_type: 1},
        Rule{direction: 1, border_type: 1},
        Rule{direction: 2, border_type: 1},
        Rule{direction: 3, border_type: 0},
    ] });
    // Setup 'SOUTH' Tile
    tiles.push(Tile { weight: 1, id: 3, value: '⊤', rules: vec![
        Rule{direction: 0, border_type: 0},
        Rule{direction: 1, border_type: 1},
        Rule{direction: 2, border_type: 1},
        Rule{direction: 3, border_type: 1},
    ] });
    // Setup 'NORTH' Tile
    tiles.push(Tile { weight: 1, id: 3, value: '⊥', rules: vec![
        Rule{direction: 0, border_type: 1},
        Rule{direction: 1, border_type: 0},
        Rule{direction: 2, border_type: 1},
        Rule{direction: 3, border_type: 1},
    ] });

    let size = 30;
    let mut wave_function_collapse = WFC::new(tiles, size);
    while !wave_function_collapse.done {
        wave_function_collapse.next_step();
    }
    wave_function_collapse.print();
}
