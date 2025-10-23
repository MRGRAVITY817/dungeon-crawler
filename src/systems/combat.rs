use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let base_damage = if let Ok(attacker_entry) = ecs.entry_ref(*attacker) {
            if let Ok(damage) = attacker_entry.get_component::<Damage>() {
                damage.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage = <(&Carried, &Damage)>::query()
            .filter(component::<Player>())
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum::<i32>();

        let final_damage = base_damage + weapon_damage;

        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;

            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}
