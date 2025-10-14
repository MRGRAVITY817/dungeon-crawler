use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    <(&mut Point, &MovingRandomly)>::query()
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
            let delta = match RandomNumberGenerator::new().range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            };
            let destination = *pos + delta;
            if map.can_enter_tile(destination) {
                *pos = destination;
            }
        });
}
