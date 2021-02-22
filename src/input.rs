use ggez::event::KeyCode;

pub struct InputState {
    xaxis: f32,
    yaxis: f32,
    fire: bool,
}

impl InputState {
    pub fn get_xaxis(&self) -> f32 {
        self.xaxis
    }
    pub fn get_yaxis(&self) -> f32 {
        self.yaxis
    }
    pub fn get_fire(&self) -> bool {
        self.fire
    }

    pub fn move_actor(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => {
                self.yaxis = 1.0;
            }
            KeyCode::Down => {
                self.yaxis = -1.0;
            }
            KeyCode::Left => {
                self.xaxis = -1.0;
            }
            KeyCode::Right => {
                self.xaxis = 1.0;
            }
            _ => (),
        }
    }
    pub fn stop_actor(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up | KeyCode::Down => {
                self.yaxis = 0.0;
            }
            KeyCode::Left | KeyCode::Right => {
                self.xaxis = 0.0;
            }
            _ => (),
        }
    }

    pub fn fire_shot(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Z => {
                self.fire = true;
            }
            _ => (),
        }
    }
    pub fn stop_shot(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Z => {
                self.fire = false;
            }
            _ => (),
        }
    }
}

impl Default for InputState {
    fn default() -> InputState {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            fire: false,
        }
    }
}
