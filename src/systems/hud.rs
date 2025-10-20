use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Point)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let player_health = <&Health>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .next()
        .unwrap();

    let mut draw_batch = DrawBatch::new();

    draw_batch.target(UI_CONSOLE_ID);

    draw_batch
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
        );

    let player = <(Entity, &Player)>::query()
        .iter(ecs)
        .map(|(entity, _)| *entity)
        .next()
        .unwrap();

    let mut item_query = <(&Item, &Name, &Carried)>::query();
    let mut y = 3;

    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)
        .for_each(|(_, name, _)| {
            draw_batch.print(Point::new(3, y), format!("{} : {}", y - 2, name.0));
            y += 1;
        });

    if y > 3 {
        draw_batch.print_color(
            Point::new(3, 2),
            "Items carried",
            ColorPair::new(YELLOW, BLACK),
        );
    }

    draw_batch.submit(10000).expect("Batch error");
}
