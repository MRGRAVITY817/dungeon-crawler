use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut renderables = <(&Point, &Render)>::query();
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).next().unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(ENTITY_CONSOLE_ID);
    let offset = Point::new(camera.left_x, camera.top_y);

    renderables
        .iter(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // Submit the batch on later order to ensure other more important batches are drawn first
    draw_batch.submit(5000).expect("Batch error");
}
