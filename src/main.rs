mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const BACKGROUND_CONSOLE_ID: usize = 1;
    pub const ENTITY_CONSOLE_ID: usize = 2;
    pub const UI_CONSOLE_ID: usize = 3;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use std::collections::HashSet;

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(UI_CONSOLE_ID);
        ctx.print_color_centered(2, RED, BLACK, "Your journey has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your adventure comes to a close.",
        );
        ctx.print_color_centered(
            6,
            WHITE,
            BLACK,
            "The amulet of Yendor remains undiscovered...",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't despair, brave soul! You can always try again.",
        );
        ctx.print_color_centered(10, GREEN, BLACK, "Press 1 to restart your quest.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(UI_CONSOLE_ID);
        ctx.print_color_centered(2, GOLD, BLACK, "You have triumphed!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "With the Amulet of Yendor in hand, you ascend from the depths.",
        );
        ctx.print_color_centered(
            6,
            WHITE,
            BLACK,
            "Songs will be sung of your heroic deeds and valor.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Celebrate your victory, brave adventurer!",
        );
        ctx.print_color_centered(10, GREEN, BLACK, "Press 1 to embark on a new quest.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        spawn_amulet(&mut self.ecs, map_builder.amulet_start);

        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn advance_level(&mut self) {
        let player_entity = <Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .next()
            .unwrap();
        let mut entities_to_keep = HashSet::new();

        // keep the player
        entities_to_keep.insert(*player_entity);

        // keep any carried items
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carried)| carried.0 == *player_entity)
            .for_each(|(entity, _)| {
                entities_to_keep.insert(*entity);
            });

        // remove all other entities
        let mut cb = CommandBuffer::new(&self.ecs);
        <Entity>::query().iter(&self.ecs).for_each(|entity| {
            if !entities_to_keep.contains(entity) {
                cb.remove(*entity);
            }
        });

        cb.flush(&mut self.ecs);

        // set field-of-view back to dirty for player
        <&mut FieldOfView>::query()
            .filter(component::<Player>())
            .iter_mut(&mut self.ecs)
            .for_each(|fov| {
                fov.is_dirty = true;
            });

        // Create new level
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        // Move player to new start position
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        // On level 2, spawn the amulet
        if map_level == 2 {
            spawn_amulet(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }

        // update resources
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
        );
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clear all consoles
        for i in [BACKGROUND_CONSOLE_ID, ENTITY_CONSOLE_ID, UI_CONSOLE_ID] {
            ctx.set_active_console(i);
            ctx.cls();
        }

        // Insert the current key and mouse position into resources
        self.resources.insert(ctx.key);
        ctx.set_active_console(BACKGROUND_CONSOLE_ID);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
            TurnState::NextLevel => {
                self.advance_level();
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
