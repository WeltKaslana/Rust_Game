use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    OverMenu,
    Home,
    InGame,
    Stop,
    None,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::Home)]
pub enum HomeState {
    #[default]
    Running,
    Pause,
    Reloading,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(HomeState = HomeState::Reloading)]
pub enum ReloadingState {
    #[default]
    Settings,
    Atlas,
    Room,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(ReloadingState = ReloadingState::Settings)]
pub enum PlayerMessageState {
    #[default]
    Shiroko,
    Arisu,
    Utaha,
    None,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, SubStates)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    Running,
    Pause,
    ChoosingBuff,
    GameOver,
}

