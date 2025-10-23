use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[write_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::Up | VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::Down | VirtualKeyCode::S => Point::new(0, 1),
            VirtualKeyCode::G => {
                // Pick up item
                let (player_entity, player_pos) = players
                    .iter(ecs)
                    .map(|(entity, pos)| (*entity, *pos))
                    .next()
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();

                items
                    .iter(ecs)
                    .filter(|(_, _item, item_pos)| **item_pos == player_pos)
                    .for_each(|(item_entity, _item, _)| {
                        commands.remove_component::<Point>(*item_entity);
                        commands.add_component(*item_entity, Carried(player_entity));

                        if let Ok(e) = ecs.entry_ref(*item_entity) {
                            // If the picked up item is a weapon, remove any other carried weapons
                            if e.get_component::<Weapon>().is_ok() {
                                <(Entity, &Carried, &Weapon)>::query()
                                    .iter(ecs)
                                    .filter(|(_, carried, _)| carried.0 == player_entity)
                                    .for_each(|(other_weapon, _, _)| {
                                        commands.remove(*other_weapon);
                                    });
                            }
                        }
                    });

                Point::zero()
            }
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            _ => Point::zero(),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .map(|(entity, pos)| (*entity, *pos + delta))
            .next()
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;

            // Try to attack an enemy at the destination
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            // If no enemy was hit, try to move
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }

        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
        });

        *turn_state = TurnState::PlayerTurn;
    }
}

/// Use an item from the player's inventory by its index.
fn use_item(inventory_index: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _)| *entity)
        .next()
        .unwrap();

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(idx, _)| *idx == inventory_index)
        .map(|(_, (item_entity, _, _))| *item_entity)
        .next();

    if let Some(item_entity) = item_entity {
        // activate item to show its effect
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }

    Point::zero()
}
