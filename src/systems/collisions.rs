use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(
    ecs: &mut SubWorld,
    // Legion provides a CommandBuffer to make changes to the world after the system has run
    // Using this, we can safely add or remove components or entities
    commands: &mut CommandBuffer,
) {
    let player_pos = <(&Point, &Player)>::query()
        .iter(ecs)
        .map(|(pos, _)| *pos)
        .next()
        .unwrap_or(Point::zero());

    <(Entity, &Point, &Enemy)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == player_pos)
        .for_each(|(entity, _, _)| {
            // Despawn the enemy entity
            commands.remove(*entity);
        });
}
