use std::collections::HashSet;

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

    fn propagate(&mut self, coord: [usize; 2]) {
        let frames_to_propagate:Vec<Option<[usize;2]>> = self.get_tiles_neighbours(coord); // Get collapsed frame neighbours
        for frame in frames_to_propagate {
            if let Some(frame_coord) = frame {
                if !self.board[frame_coord[0]][frame_coord[1]].collapsed {
                    let neighbours:Vec<Option<[usize;2]>> = self.get_tiles_neighbours(frame_coord); // For each of them non collapsed, get neighbours
                    for (direction, neighbour) in neighbours.iter().enumerate() {
                        if let Some(neighbour_coord) = neighbour {
                            self.remove_tile_options(
                                direction,
                                frame_coord,
                                self.board[neighbour_coord[0]][neighbour_coord[1]].clone());
                        }
                    }
                }
            }
        }
    }
    fn remove_tile_options(&mut self, direction: usize, frame_coord: [usize;2], neighbour: Frame) {
        let opposite_direction = self.get_opposite_direction(direction);
        let mut frame = &mut self.board[frame_coord[0]][frame_coord[1]];
        let neighbour_authorized_border_types: Vec<String> = neighbour.options.into_iter()
            .map(|t| {
                    t.rules[opposite_direction].border_type.clone()
                })
            .collect::<HashSet<_>>() // Filter to have only uniq border_types
            .into_iter()
            .collect();
        // Filter options of frame if still possible
        let new_options: Vec<Tile> = frame.options.clone()
            .into_iter()
            .filter(|o| (neighbour_authorized_border_types
                .iter()
                .any(|bt| {
                    return bt.eq(&o.rules[direction].border_type)
                    })
                )
            )
            .collect();
        if new_options.len() == 0 { panic!("One frame has no option left")} // How to handle that
        frame.options = new_options; 
            
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
    fn collapse(&mut self) -> Option<[usize;2]>{
        match self.get_lowest_entropy() {
            Some(coord) => {
                let frame = &mut self.board[coord[0]][coord[1]]; // Get frame with coordinates
                let weights: Vec<u32> = frame.options.iter().map(|option| option.weight).collect(); // Extract weights from frame options
                let dist = WeightedIndex::new(&weights).unwrap();
                let mut rng = thread_rng();
                let chosen_tile = frame.options.clone()[dist.sample(&mut rng)].clone(); // Choice of a random option taking into account the weights
                frame.collapse(vec![chosen_tile]);
                return Some(coord)
            },
            None => {
                self.done = true;
                return None;
            }
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
            match self.collapse() {
                Some(coord) => self.propagate(coord),
                None => {}
            };
            
        }
        return self.board.clone();

    }

}
