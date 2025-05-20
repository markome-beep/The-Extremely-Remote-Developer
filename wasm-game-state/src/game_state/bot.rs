use serde::{Deserialize, Serialize};
use tsify_next::Tsify;

use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Copy, Clone)]
enum Action {
    Move,
    TurnL,
    TurnR,
    Wait,
}

#[derive(Tsify, Serialize, Copy, Clone, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub enum BotState {
    Moving { end_x: i32, end_y: i32 },
    Paused,
    Turning { end_dir: Dir },
    Start,
    Done,
}

#[wasm_bindgen]
#[derive(Copy, Clone, Serialize, Deserialize)]
#[repr(u8)]
pub enum Dir {
    N = 0,
    NW = 1,
    SW = 2,
    S = 3,
    SE = 4,
    NE = 5,
}

#[wasm_bindgen]
pub struct Bot {
    actions: Vec<Action>,
    curr: usize,
    lite: BotLite,
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct BotLite {
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
    pub state: BotState,
    pub id: i32,
}

#[wasm_bindgen]
impl Bot {
    pub fn new(x: i32, y: i32, id: i32) -> Self {
        use Action::*;
        Self {
            actions: vec![Move, TurnL, TurnR, Move, Wait, Move, TurnL, Move, Move],
            curr: 0,
            lite: BotLite {
                x,
                y,
                id,
                dir: Dir::N,
                state: BotState::Start,
            },
        }
    }
    pub fn tick(&mut self) -> BotLite {
        use BotState::*;

        match self.lite.state {
            Moving { end_x, end_y, .. } => {
                self.lite.x = end_x;
                self.lite.y = end_y;
            }
            Turning { end_dir, .. } => {
                self.lite.dir = end_dir;
            }
            _ => {}
        }
        self.next_state();
        self.lite
    }

    fn next_state(&mut self) {
        use BotState::*;

        if self.curr >= self.actions.len() {
            self.lite.state = BotState::Done;
            return;
        }

        self.lite.state = match &self.actions[self.curr] {
            Action::Move => match self.lite.dir {
                Dir::N => Moving {
                    end_x: self.lite.x,
                    end_y: self.lite.y + 1,
                },
                Dir::NW => Moving {
                    end_x: self.lite.x - 1,
                    end_y: self.lite.y,
                },
                Dir::SW => Moving {
                    end_x: self.lite.x - 1,
                    end_y: self.lite.y - 1,
                },
                Dir::S => Moving {
                    end_x: self.lite.x,
                    end_y: self.lite.y - 1,
                },
                Dir::SE => Moving {
                    end_x: self.lite.x + 1,
                    end_y: self.lite.y,
                },
                Dir::NE => Moving {
                    end_x: self.lite.x + 1,
                    end_y: self.lite.y + 1,
                },
            },
            Action::TurnR => match self.lite.dir {
                Dir::N => Turning { end_dir: Dir::NW },
                Dir::NW => Turning { end_dir: Dir::SW },
                Dir::SW => Turning { end_dir: Dir::S },
                Dir::S => Turning { end_dir: Dir::SE },
                Dir::SE => Turning { end_dir: Dir::NE },
                Dir::NE => Turning { end_dir: Dir::N },
            },
            Action::TurnL => match self.lite.dir {
                Dir::N => Turning { end_dir: Dir::NE },
                Dir::NW => Turning { end_dir: Dir::N },
                Dir::SW => Turning { end_dir: Dir::NW },
                Dir::S => Turning { end_dir: Dir::SW },
                Dir::SE => Turning { end_dir: Dir::S },
                Dir::NE => Turning { end_dir: Dir::SE },
            },
            Action::Wait => Moving {
                end_x: self.lite.x,
                end_y: self.lite.y,
            },
        };

        self.curr += 1;
    }
}
