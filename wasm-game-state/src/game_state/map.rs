use web_sys::js_sys;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum TileKinds {
    Empty = 0,
    Grass = 1,
    Path = 3,
    Wall = 4,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Tile {
    pub kind: TileKinds,
}

pub static CHUNK_SIZE: u8 = 10;

pub struct Chunk {
    pub tiles: Vec<Tile>,
}

impl Chunk {
    pub fn new() -> Self {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE).into());

        for _x in 0..=CHUNK_SIZE {
            for _y in 0..=CHUNK_SIZE {
                tiles.push(Tile {
                    kind: TileKinds::Grass,
                });
            }
        }

        Self { tiles }
    }

    pub fn random_paths() -> Self {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE).into());

        for x in 0..=CHUNK_SIZE {
            for y in 0..=CHUNK_SIZE {
                if x == 0 && y == 0 {
                    tiles.push(Tile {
                        kind: TileKinds::Path,
                    });
                } else if 0.25 > js_sys::Math::random() {
                    tiles.push(Tile {
                        kind: TileKinds::Path,
                    });
                } else {
                    tiles.push(Tile {
                        kind: TileKinds::Wall,
                    });
                }
            }
        }

        Self { tiles }
    }
}
