#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AppScreen {
    Splash,
    MainMenu,
    InGame,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameStatus {
    Tutorial,
    Running,
    Paused,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GameMode {
    SinglePlayer,
    MultiPlayer,
}
