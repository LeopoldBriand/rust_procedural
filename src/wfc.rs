use rand::prelude::*;
use rand::distributions::WeightedIndex;
use serde::{Deserialize, Serialize};

use crate::tileset::Tileset;

#[derive(Clone, Serialize, Deserialize)]
pub struct Rule {
    pub direction: usize,
    pub border_type: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Tile {
    pub weight: u32,
    pub id: i32,
    pub value: u32, // Position of the tile in tile_sheet image 
    pub rules: Vec<Rule>
}

#[derive(Clone)]
pub struct Frame {
    coord: [usize; 2],
    pub collapsed: bool,
    pub options: Vec<Tile>
}
impl Frame {
    pub fn collapse(&mut self, new_options: Vec<Tile>) {
        self.options = new_options;
        self.collapsed = true;
    }
}
#[derive(Clone)]
pub struct WFC {
    tiles: Vec<Tile>,
    board: Vec<Vec<Frame>>,
    size: usize,
    pub done: bool
}

impl WFC {
    pub fn new(tileset: Tileset, size: usize) -> Self {
        let mut board = Vec::new();
        for x in 0..size {
            board.push(Vec::new());
            for y in 0..size {
                board[x].push(Frame {coord: [x,y], collapsed: false, options: tileset.tiles.clone()});
            }
        }
        
        return WFC { tiles: tileset.tiles, board: board, done: false, size: size};
    }
    fn propagate(&mut self){
        for (x, line) in self.board.clone().iter().enumerate() {
            for (y, _frame) in line.iter().enumerate() {
                
                let neighbours:Vec<Option<[usize;2]>> = self.get_tiles_neighbours([x,y]);
                for (direction, neighbour) in neighbours.iter().enumerate() { // Get all neighbours options
                    if !self.board[x][y].collapsed {
                        match neighbour {
                            Some(coord) => {
                                let neighbours_options: Vec<Tile> = self.board[coord[0]][coord[1]].options.clone();
                                let opposite_direction = self.get_opposite_direction(direction);
                                let neighbours_authorized_border_types: Vec<String> = neighbours_options.into_iter().map(|t| {
                                        t.rules[opposite_direction].border_type.clone()
                                    }).collect();
                                // Check if every options of frame if still possible
                                let mut index_to_remove = Vec::new();
                                for (index, option) in self.board[x][y].options.clone().into_iter().enumerate() {
                                    if !(neighbours_authorized_border_types.iter().any(|bt| bt.eq(&option.rules[direction].border_type))) {
                                        // Remove unauthorized option
                                        index_to_remove.push(index);
                                    } 
                                }
                                index_to_remove.sort_by(|a, b| b.cmp(a));
                                for index in index_to_remove {
                                    self.board[x][y].options.remove(index);
                                    if self.board[x][y].options.len() == 1 {self.board[x][y].collapsed = true; break;} // Not sure about this one ...
                                }
                            },
                            None => {}
                        }
                    }
                }
            }
        }
    }
    fn get_tiles_neighbours(&self, coord: [usize; 2]) -> Vec<Option<[usize; 2]>> {
        let mut neighbours: Vec<Option<[usize; 2]>> = Vec::new();
        // Get north tile
        if coord[0] == 0 {
            neighbours.push(None)
        } else {
            neighbours.push(Some([coord[0]-1, coord[1]]));
        }
        // Get south tile
        if coord[0] == self.size -1 {
            neighbours.push(None)
        } else {
            neighbours.push(Some([coord[0]+1, coord[1]]));
        }
        // Get west tile
        if coord[1] == 0 {
            neighbours.push(None)
        } else {
            neighbours.push(Some([coord[0], coord[1]-1]));
        }
        // Get east tile
        if coord[1] == self.size -1 {
            neighbours.push(None)
        } else {
            neighbours.push(Some([coord[0], coord[1]+1]));
        }
        return neighbours;
    }
    fn get_opposite_direction(&self, direction: usize) -> usize {
        match direction {
            0 => 1, // north -> south
            1 => 0, // south -> north
            2 => 3, // west -> east
            3 => 2, // east -> west
            _ => {
                panic!("unknow direction")
            }
        }
    }
    fn collapse(&mut self){
        match self.get_lowest_entropy() {
            Some(coord) => {
                let frame = &mut self.board[coord[0]][coord[1]]; // Get frame with coordinates
                let weights: Vec<u32> = frame.options.iter().map(|option| option.weight).collect(); // Extract weights from frame options
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = thread_rng();
                let chosen_tile = frame.options.clone()[dist.sample(&mut rng)].clone(); // Choice of a random option taking into account the weights
                frame.collapse(vec![chosen_tile]);
            },
            None => {self.done = true}
        }
    }
    fn get_lowest_entropy(&mut self) -> Option<[usize; 2]> {
        let mut minimum_entropy = self.tiles.len(); // Initialize minimal entropy as tileset length
        let mut chosen_frames: Vec<&Frame> = Vec::new();
        for (_i, el) in self.board.iter().enumerate() {
            for (_j, frame) in el.iter().enumerate() {
                if !frame.collapsed {
                    if frame.options.len() < minimum_entropy {
                        chosen_frames = vec![frame]; // If a frame as less entropy, reinitialize possible choices
                        minimum_entropy = frame.options.len();
                    }
                    if frame.options.len() == minimum_entropy {
                        chosen_frames.push(frame); // If some frame already as the same entropy, push to the possible choices
                    }
                }
            }
        }
        if chosen_frames.len() > 0 {
            let mut rng = rand::thread_rng();
            let size = chosen_frames.len() as i32;
            let r: i32 = rng.gen_range(0..size); // Choose the frame to collapse randomly
            return Some(chosen_frames[r as usize].coord)
        } else {
            return None; // All Frames are collapsed
        }
    }
    pub fn resolve(&mut self) -> Vec<Vec<Frame>> {
        while !self.done {
            self.collapse();
            self.propagate();
        }
        return self.board.clone();

    }

}
