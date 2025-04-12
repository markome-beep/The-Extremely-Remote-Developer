use std::collections::HashMap;
use web_sys::js_sys;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils;

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

static CHUNK_SIZE: u8 = 10;
#[derive(Clone)]
struct Chunk {
    tiles: Vec<Tile>,
}

impl Chunk {
    fn new() -> Self {
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

    fn random_paths() -> Self {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE).into());

        for _x in 0..=CHUNK_SIZE {
            for _y in 0..=CHUNK_SIZE {
                if 0.25 > js_sys::Math::random() {
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

enum ChunkGen {
    RandomPath,
    Grass,
}

#[wasm_bindgen]
struct GameData {
    map: HashMap<(i32, i32), Chunk>,
    map_gen: ChunkGen,
}

#[wasm_bindgen]
#[allow(unused)]
impl GameData {
    pub fn new() -> Self {
        utils::set_panic_hook();

        let mut map = HashMap::new();
        map.insert((0, 0), Chunk::new());
        Self {
            map,
            map_gen: ChunkGen::Grass,
        }
    }

    pub fn paths() -> Self {
        utils::set_panic_hook();

        let mut map = HashMap::new();
        map.insert((0, 0), Chunk::random_paths());
        Self {
            map,
            map_gen: ChunkGen::RandomPath,
        }
    }

    pub fn get_tile(&mut self, x: i32, y: i32) -> Tile {
        let chunk = (
            x.div_euclid(CHUNK_SIZE as i32),
            y.div_euclid(CHUNK_SIZE as i32),
        );

        let (local_x, local_y) = (
            x.rem_euclid(CHUNK_SIZE as i32),
            y.rem_euclid(CHUNK_SIZE as i32),
        );

        self.map
            .entry(chunk)
            .or_insert(match self.map_gen {
                ChunkGen::RandomPath => Chunk::random_paths(),
                ChunkGen::Grass => Chunk::new(),
            })
            .tiles[(local_x + local_y * CHUNK_SIZE as i32) as usize]
    }
}
