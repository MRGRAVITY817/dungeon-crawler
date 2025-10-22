mod template;

use crate::{prelude::*, spawner::template::Templates};

/// Spawns the player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player { map_level: 0 },
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
        Health {
            current: 100,
            max: 100,
        },
        FieldOfView::new(8),
    ));
}

/// Spawns the Amulet of Yendor at the given position
pub fn spawn_amulet(ecs: &mut World, pos: Point) {
    ecs.push((
        Item,
        AmuletOfYendor,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('|'),
        },
        Name("Amulet of Yendor".to_string()),
    ));
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    map_level: usize,
    spawn_points: &[Point],
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, map_level, spawn_points);
}
