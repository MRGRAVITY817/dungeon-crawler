use crate::prelude::*;

/// Spawns the player entity at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}

/// Spawns a monster entity at the given position with a random glyph
pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, pos: Point) {
    ecs.push((
        Enemy,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: match rng.range(0, 4) {
                0 => to_cp437('E'), // Ettin (two-headed giant)
                1 => to_cp437('O'), // Ogre
                2 => to_cp437('o'), // Orc
                _ => to_cp437('g'), // Goblin
            },
        },
        MovingRandomly,
    ));
}
