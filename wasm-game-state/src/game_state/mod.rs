use std::collections::HashMap;

use bot::{Bot, BotLite};
use map::{CHUNK_SIZE, Chunk, Tile};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils;
mod bot;
mod map;

enum ChunkGen {
    RandomPath,
    Grass,
}

#[wasm_bindgen]
struct GameData {
    map: HashMap<(i32, i32), Chunk>,
    map_gen: ChunkGen,
    bot: Vec<Bot>,
    bot_count: i32,
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
            bot: vec![],
            bot_count: 1,
        }
    }

    pub fn paths() -> Self {
        utils::set_panic_hook();

        let mut map = HashMap::new();
        map.insert((0, 0), Chunk::random_paths());
        Self {
            map,
            map_gen: ChunkGen::RandomPath,
            bot: vec![],
            bot_count: 1,
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

    pub fn tick(&mut self) -> Vec<BotLite> {
        self.bot.iter_mut().map(|b| b.tick()).collect()
    }

    pub fn add_bot(&mut self) -> BotLite {
        let mut b = Bot::new(0, 0, self.bot_count);
        self.bot_count += 1;
        self.bot.push(b);
        self.bot.last_mut().unwrap().tick()
    }
}
