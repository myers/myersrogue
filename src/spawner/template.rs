use crate::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
// use std::fs::File;

const TEMPLATE_FILE: &[u8] = include_bytes!("../../resources/template.ron");

#[derive(Deserialize, Clone, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
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
        // let file = File::open("resources/templates.ron").expect("failed to open templates.ron");
        // from_reader(file).expect("unable to parse templates.ron")
        from_reader(TEMPLATE_FILE).expect("unable to parse templates.ron")
    }
    pub fn spawn_entities(
        &self,
        world: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|t| t.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });
        // available_entities.sort_by_cached_key(|f| rng.range(0, available_entities.len()));

        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, world);
            }
        });
    }

    pub fn spawn_entity(&self, pt: &Point, template: &Template, world: &mut World) {
        let mut entity = world.spawn((
            PointC(*pt),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph),
            },
            Name(template.name.clone()),
        ));

        match template.entity_type {
            EntityType::Item => {
                entity.insert(Item {});
            }
            EntityType::Enemy => {
                entity.insert(Enemy {});
                entity.insert(FieldOfView::new(6));
                entity.insert(ChasingPlayer {});
                entity.insert(Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap(),
                });
            }
        }

        if let Some(effects) = &template.provides {
            for (provides, n) in effects.iter() {
                match provides.as_str() {
                    "Healing" => {
                        entity.insert(ProvidesHealing { amount: *n });
                    }
                    "MagicMap" => {
                        entity.insert(ProvidesDungeonMap {});
                    }
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                }
            }
        }
        if let Some(damage) = &template.base_damage {
            entity.insert(Damage(*damage));
            if template.entity_type == EntityType::Item {
                entity.insert(Weapon {});
            }
        }
    }
}
