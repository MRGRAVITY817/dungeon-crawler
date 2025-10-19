use super::MapArchitect;
use crate::prelude::*;
pub struct DrunkardArchitect {}

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;
const STAGGER_DISTANCE: i32 = 400;

impl MapArchitect for DrunkardArchitect {
    fn design(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        // Step 1: Fill the map with walls
        mb.fill(TileType::Wall);

        // Step 2: Perform drunkard's walk until enough floor tiles are created
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard_walk(&center, rng, &mut mb.map);

        while mb
            .map
            .tiles
            .iter()
            .filter(|t| **t == TileType::Floor)
            .count()
            < DESIRED_FLOOR
        {
            // Start a new drunkard at a random position
            self.drunkard_walk(
                &Point::new(rng.range(0, SCREEN_WIDTH), rng.range(0, SCREEN_HEIGHT)),
                rng,
                &mut mb.map,
            );

            let dijkstra_map = DijkstraMap::new(
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &[mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0,
            );

            // Remove distant floor tiles to make more natural caves
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|&(_, dist)| *dist > 2000.0)
                .for_each(|(idx, _)| {
                    mb.map.tiles[idx] = TileType::Wall;
                });
        }

        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();

        mb
    }
}

impl DrunkardArchitect {
    /// Carves out the map using the drunkard's walk algorithm
    fn drunkard_walk(&mut self, start: &Point, rng: &mut RandomNumberGenerator, map: &mut Map) {
        let mut drunkard_pos = *start;
        let mut distance_staggered = 0;

        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);

            // Carve out a floor tile at the drunkard's current position
            map.tiles[drunk_idx] = TileType::Floor;

            // Decide next direction to walk
            match rng.range(0, 4) {
                0 => drunkard_pos.x += 1,
                1 => drunkard_pos.x -= 1,
                2 => drunkard_pos.y += 1,
                _ => drunkard_pos.y -= 1,
            }

            // If the drunkard hit's the edge of the map, stop walking
            if !map.in_bounds(drunkard_pos) {
                break;
            }

            distance_staggered += 1;

            // If the drunkard has walked far enough, stop walking
            if distance_staggered > STAGGER_DISTANCE {
                break;
            }
        }
    }
}
