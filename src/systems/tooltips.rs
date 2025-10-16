use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
pub fn tooltips(ecs: &SubWorld, #[resource] mouse_pos: &Point, #[resource] camera: &Camera) {
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(UI_CONSOLE_ID);

    <(Entity, &Point, &Name)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos)
        .for_each(|(entity, _, name)| {
            // Since the tooltip layer is 4x the resolution of the map layer (see main.rs),
            // we need to scale the position to match that.
            let screen_pos = *mouse_pos * 4;
            let display =
                if let Ok(health) = ecs.entry_ref(*entity).unwrap().get_component::<Health>() {
                    format!("{} ({} HP)", name.0, health.current)
                } else {
                    name.0.clone()
                };
            draw_batch.print(screen_pos, display);
        });

    draw_batch.submit(10100).expect("Batch error");
}
