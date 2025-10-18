use crate::prelude::*;

#[system(for_each)] // `for_each` to run the system for every matching entity
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        // Mark as `wants to move` for every entity that wants to move
        commands.add_component(want_move.entity, want_move.destination);

        // If that entity is the player, update the camera as well
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());
            }
            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(want_move.destination);
            }
        }
    }

    commands.remove(*entity); // Remove the `WantsToMove` component after processing
}
