#[derive(Default)]
pub struct PlayerCommand {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerCommand {
    pub fn key_command(&mut self, action: &str) {
        if action == "MOVE_FORWARD" {
            self.forward = true;
        }
        if action == "MOVE_BACKWARD" {
            self.backward = true;
        }
        if action == "MOVE_LEFT" {
            self.left = true;
        }
        if action == "MOVE_RIGHT" {
            self.right = true;
        }
    }

    pub fn mouse_command(&mut self, action: &str) {
        if action == "MOUSE_SHOOT" {
            bevy::log::info!("Bang");
        }
        if action == "AIM_UP" {}
        if action == "AIM_DOWN" {}
        if action == "AIM_LEFT" {}
        if action == "AIM_RIGHT" {}
    }

    pub fn reset(&mut self) {
        self.forward = false;
        self.backward = false;
        self.left = false;
        self.right = false;
    }
}
