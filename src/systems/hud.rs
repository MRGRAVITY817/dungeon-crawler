use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    DrawBatch::new()
        .target(UI_CONSOLE_ID)
        .print_centered(1, "Explore the Dungeon. Cursor keys to move.")
        .bar_horizontal(
            Point::zero(),
            SCREEN_WIDTH * 2,
            player_health.current,
            player_health.max,
            ColorPair::new(RED, BLACK),
        )
        .print_color_centered(
            0,
            format!("Health: {} / {}", player_health.current, player_health.max),
            ColorPair::new(WHITE, RED),
        )
        .submit(10000)
        .expect("Batch error");
}
