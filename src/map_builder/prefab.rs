use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = (
    "
------------
---######---
---#----#---
---#-M--#--- 
-###----###- 
--M------M-- 
-###----###- 
---#----#--- 
---#----#--- 
---######--- 
------------
",
    12,
    11,
);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placements = None;

    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &[mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0,
    );

    let mut attempts = 0;
    while placements.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, SCREEN_WIDTH - FORTRESS.1),
            rng.range(0, SCREEN_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        let mut can_place = false;

        dimensions.for_each(|p| {
            let idx = mb.map.point2d_to_index(p);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0     // Ensure it's reachable
            && distance > 20.0       // Ensure it's not overlapping with player start
            && mb.amulet_start != p
            // Ensure it's not overlapping with amulet start
            {
                can_place = true;
            }
        });

        if can_place {
            placements = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|spawn| !points.contains(spawn));
        }

        attempts += 1;
    }

    if let Some(origin) = placements {
        for (y, line) in FORTRESS.0.trim().lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let map_x = origin.x + x as i32;
                let map_y = origin.y + y as i32;
                let idx = map_idx(map_x, map_y);
                match ch {
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(map_x, map_y));
                    }
                    _ => {}
                }
            }
        }
    }
}
