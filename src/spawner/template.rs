use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: u32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed to open templates file");

        from_reader(file).expect("Failed to parse templates file")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        map_level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&map_level))
            .for_each(|e| {
                for _ in 0..e.frequency {
                    available_entities.push(e.clone());
                }
            });

        println!(
            "Spawning from {} available entities",
            available_entities.len()
        );

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|&pos| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                println!("Spawning entity {:?} at {:?}", entity.name, pos);
                self.spawn_entity(pos, entity, &mut commands);
            }
        });
        commands.flush(ecs);
    }

    fn spawn_entity(&self, pos: Point, template: &Template, commands: &mut CommandBuffer) {
        // Render the entity
        let entity = commands.push((
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        // Add components based on entity type
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer);
                commands.add_component(
                    entity,
                    Health {
                        current: template.hp.unwrap_or(1),
                        max: template.hp.unwrap_or(1),
                    },
                );
            }
        }

        // If the entity provides effects, add those components
        if let Some(effects) = &template.provides {
            for (effect, value) in effects {
                match effect.as_str() {
                    "Healing" => {
                        commands.add_component(entity, ProvidesHealing { amount: *value });
                    }
                    "DungeonMap" => {
                        commands.add_component(entity, ProvidesDungeonMap);
                    }
                    _ => {
                        println!("Unknown effect: {}", effect);
                    }
                }
            }
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon);
            }
        }
    }
}
