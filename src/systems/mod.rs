use crate::prelude::*;

mod chasing;
mod combat;
mod end_turn;
mod entity_render;
mod fov;
mod hud;
mod map_render;
mod movement;
mod player_input;
mod random_move;
mod tooltips;
mod use_items;

pub fn build_system_sets(app: &mut App) {
    use GameStage::*;
    use TurnState::*;

    app.add_system_set(
        ConditionSet::new()
            .label(StateLabel::Fov)
            .run_unless_resource_equals(GameOver)
            .run_unless_resource_equals(Start)
            .with_system(fov::fov)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_unless_resource_equals(GameOver)
            .run_unless_resource_equals(Start)
            .after(StateLabel::Fov)
            .with_system(map_render::map_render)
            .with_system(entity_render::entity_render)
            .with_system(hud::hud)
            // In the source project, the tooltips system is run only in the player input frames.
            // Here, due to the different design, it's executed on every frame.
            .with_system(tooltips::tooltips)
            .into(),
    );

    app.add_system_set(
        ConditionSet::new()
            .run_if_resource_equals(AwaitingInput)
            .with_system(player_input::player_input)
            .into(),
    );

    app.add_system_set_to_stage(
        PlayerCombat,
        ConditionSet::new()
            .run_if_resource_equals(PlayerTurn)
            .with_system(use_items::use_items)
            .with_system(combat::combat)
            .into(),
    );

    app.add_system_set_to_stage(
        MovePlayer,
        ConditionSet::new()
            .run_if_resource_equals(PlayerTurn)
            .with_system(movement::movement)
            .with_system(end_turn::end_turn)
            .into(),
    );

    app.add_system_set_to_stage(
        PlayerFov,
        ConditionSet::new()
            .run_if_resource_equals(PlayerTurn)
            .with_system(fov::fov)
            .into(),
    );

    app.add_system_set_to_stage(
        GenerateMonsterMoves,
        ConditionSet::new()
            .run_if_resource_equals(MonsterTurn)
            .with_system(random_move::random_move)
            .with_system(chasing::chasing)
            .into(),
    );

    app.add_system_set_to_stage(
        MonsterCombat,
        ConditionSet::new()
            .run_if_resource_equals(MonsterTurn)
            .with_system(use_items::use_items)
            .with_system(combat::combat)
            .into(),
    );

    app.add_system_set_to_stage(
        MoveMonsters,
        ConditionSet::new()
            .run_if_resource_equals(MonsterTurn)
            .with_system(movement::movement)
            .with_system(end_turn::end_turn)
            .into(),
    );

    app.add_system_set_to_stage(
        MonsterFov,
        ConditionSet::new()
            .run_if_resource_equals(MonsterTurn)
            .with_system(fov::fov)
            .into(),
    );
}
