use std::iter::zip;

use super::MapArchitect;
use crate::prelude::*;

/// An architect that uses cellular automata to generate cave-like maps
///
/// ## What is Cellular Automata?
/// Cellular automata are discrete, abstract computational systems that have found
/// application in various fields, including computer science, physics, and biology.
///
/// They consist of a grid of cells, each of which can be in one of a finite number of states.
/// The state of each cell at the next time step is determined by a set of rules that consider the states of neighboring cells.
///
/// In the context of map generation, cellular automata can be used to create organic, cave-like structures
/// by iteratively applying rules that simulate natural processes such as erosion and growth.
/// This method is particularly effective for generating maps that require a more natural and less structured appearance,
/// such as caves or wilderness areas.
pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn design(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Step 1: Initialize the map with random noise
        self.random_noise_map(rng, &mut mb.map);

        // Step 2: Apply cellular automata rules for a number of iterations
        // More iterations lead to more defined structures
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }

        // Step 3: Determine player start and amulet positions
        let player_start = self.find_start(&mb.map);
        mb.player_start = player_start;
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.spawn_monsters(&player_start, rng);

        mb
    }
}

impl CellularAutomataArchitect {
    /// Fills the map with random noise as a starting point for cellular automata
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll < 55 {
                *t = TileType::Wall;
            } else {
                *t = TileType::Floor;
            }
        });
    }

    /// Count the number of wall tiles surrounding a given tile
    fn count_wall_neighbors(&self, x: i32, y: i32, tiles: &[TileType]) -> i32 {
        zip(-1..=1, -1..=1)
            .filter(|(dy, dx)| !(*dy == 0 && *dx == 0))
            .filter(|(dy, dx)| {
                let neighbor_x = x + *dx;
                let neighbor_y = y + *dy;
                tiles[map_idx(neighbor_x, neighbor_y)] == TileType::Wall
            })
            .count() as i32
    }

    /// Apply cellular automata rules to the map for a number of iterations
    fn iteration(&mut self, map: &mut Map) {
        let map_tiles = map.tiles.clone();

        let new_tiles = zip(1..SCREEN_WIDTH - 1, 1..SCREEN_HEIGHT - 1).map(|(x, y)| {
            let neighbors = self.count_wall_neighbors(x, y, &map_tiles);
            let idx = map_idx(x, y);

            let tile_type = if neighbors > 4 || neighbors == 0 {
                TileType::Wall
            } else {
                TileType::Floor
            };

            (idx, tile_type)
        });

        for (idx, tile_type) in new_tiles {
            map.tiles[idx] = tile_type;
        }
    }

    /// Find a suitable starting position for the player
    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

        map.tiles
            .iter()
            .enumerate()
            // Only consider floor tiles where the player can start
            .filter(|&(_, &tile)| tile == TileType::Floor)
            // Find the closest floor tile to the center
            .min_by_key(|&(idx, _)| {
                let point = map.index_to_point2d(idx);
                DistanceAlg::Pythagoras.distance2d(center, point) as i32
            })
            // Convert the index back to a Point
            .map(|(idx, _)| map.index_to_point2d(idx))
            .unwrap_or(center)
    }
}
