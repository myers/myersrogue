use bevy::prelude::Resource;

#[derive(Copy, Clone, Debug, PartialEq, Resource)]
pub enum TurnState {
    Start,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
