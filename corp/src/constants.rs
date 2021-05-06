pub mod tick {
    pub const ONE_SECOND: f64 = 1.0;
    pub const FRAME_RATE: f64 = 60.0;
    pub const TIME_STEP: f32 = (ONE_SECOND / FRAME_RATE) as f32;
}

pub mod window {
    pub const CORP_ONE_GAME_TITLE: &str = "Corp One";
    pub const WIDTH: f32 = 1600.0;
    pub const HEIGHT: f32 = 1600.0;
}

pub mod state {
    #[derive(Debug, Hash, PartialEq, Eq, Clone)]
    pub enum GameState {
        Loading,
        _StarMap,
        Playing,
    }
}

pub mod input {
    pub const MOVE_FORWARD: &str = "MOVE_FORWARD";
    pub const MOVE_BACKWARD: &str = "MOVE_BACKWARD";
    pub const MOVE_RIGHT: &str = "MOVE_RIGHT";
    pub const MOVE_LEFT: &str = "MOVE_LEFT";
    pub const MOUSE_SHOOT: &str = "MOUSE_SHOOT";
}
