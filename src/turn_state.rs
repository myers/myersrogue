#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    Start,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
