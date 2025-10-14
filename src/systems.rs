mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use crate::prelude::*;

pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        // commands executed in collisions need to be applied before random_move runs
        // hence we flush, which waits for all previous systems to complete
        // fyi, it's always good to flush whenever you have a system that uses CommandBuffer
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}
