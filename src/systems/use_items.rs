use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    // list of healing effects to apply
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    // Process all activated items
    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            if let Ok(entry) = ecs.entry_ref(activate.item) {
                // Check for healing
                if let Ok(healing) = entry.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                // Check for dungeon map reveal
                if entry.get_component::<ProvidesDungeonMap>().is_ok() {
                    map.revealed_tiles.iter_mut().for_each(|tile| *tile = true);
                }
            }

            // Remove the used item from the game
            commands.remove(activate.item);
            commands.remove(*entity);
        });

    // Apply healing effects
    for (heal_reciever, amount) in healing_to_apply {
        if let Ok(mut entry) = ecs.entry_mut(heal_reciever) {
            if let Ok(health) = entry.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + amount);
            }
        }
    }
}
