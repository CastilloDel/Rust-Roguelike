use rltk::{Algorithm2D, BaseMap, Point};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub depth: i32,
    pub bloodstains: HashSet<usize>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, index: usize) -> bool {
        self.tiles[index as usize] == TileType::Wall
    }

    fn get_available_exits(&self, index: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = index as i32 % self.width;
        let y = index as i32 / self.width;
        let w = self.width as usize;
        // Cardinal directions
        if self.is_exit_valid(x - 1, y) {
            exits.push((index - 1, 1.0))
        };
        if self.is_exit_valid(x + 1, y) {
            exits.push((index + 1, 1.0))
        };
        if self.is_exit_valid(x, y - 1) {
            exits.push((index - w, 1.0))
        };
        if self.is_exit_valid(x, y + 1) {
            exits.push((index + w, 1.0))
        };

        // Diagonals
        if self.is_exit_valid(x - 1, y - 1) {
            exits.push(((index - w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y - 1) {
            exits.push(((index - w) + 1, 1.45));
        }
        if self.is_exit_valid(x - 1, y + 1) {
            exits.push(((index + w) - 1, 1.45));
        }
        if self.is_exit_valid(x + 1, y + 1) {
            exits.push(((index + w) + 1, 1.45));
        }

        exits
    }

    fn get_pathing_distance(&self, index1: usize, index2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(index1 % w, index1 / w);
        let p2 = Point::new(index2 % w, index2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

impl Map {
    pub fn xy_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let index = self.xy_index(x, y);
        !self.blocked[index]
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new(new_depth: i32, width: i32, height: i32) -> Map {
        let map_tile_count = (width * height) as usize;
        Map {
            tiles: vec![TileType::Wall; map_tile_count],
            width,
            height,
            revealed_tiles: vec![false; map_tile_count],
            visible_tiles: vec![false; map_tile_count],
            blocked: vec![false; map_tile_count],
            tile_content: vec![Vec::new(); map_tile_count],
            depth: new_depth,
            bloodstains: HashSet::new(),
        }
    }
}
