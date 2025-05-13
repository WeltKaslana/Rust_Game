use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    OverMenu,
    Home,
    InGame,
    Stop,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Home)]
pub enum HomeState {
    #[default]
    Running,
    Pause,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    Running,
    Pause,
    Reloading,
}

