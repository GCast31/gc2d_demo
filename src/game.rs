
use std::collections::HashMap;

use gc2d::{event::EventLoop, color::Color};

use crate::tilemap::{TileMap, TypeTileMap};

const MAP_TILE_HEIGHT: usize = 70;
const MAP_TILE_WIDTH: usize = 70;

pub struct Game {
    map: TileMap<u32>,
}

impl Game {
    pub fn new() -> Self {

        let map_tiles = HashMap::from([
            (1, TypeTileMap::FromSimpleFile("assets/images/grassCenter.png".to_string())),
            (2, TypeTileMap::FromSimpleFile("assets/images/liquidLava.png".to_string())),
            (3, TypeTileMap::FromSimpleFile("assets/images/liquidWater.png".to_string())),
        ]);

        Self {
            map: TileMap::new(
                map_tiles,
                MAP_TILE_HEIGHT,
                MAP_TILE_WIDTH,
            ) 
        }
    }

    pub fn load_map1(&mut self) {
        self.map.set_map(Some(vec![
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 2],
            vec![2, 2, 3, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 3, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 3, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 0, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 1, 2],
            vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        ]));
    }
}

impl EventLoop for Game {

    fn load(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, _audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        gc2d.window.set_title("Demo");
        gc2d.window.set_size(700., 700.);

        gc2d.graphics.new_image("assets/images/grassCenter.png").unwrap();
        gc2d.graphics.new_image("assets/images/liquidLava.png").unwrap();
        gc2d.graphics.new_image("assets/images/liquidWater.png").unwrap();

        gc2d.graphics.new_font("assets/fonts/PixelMaster.ttf", 20);

        self.load_map1();

        Ok(())
    }

    fn draw(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, fonts: &mut gc2d::fonts::FontsManager, dt: f32) -> Result<(), gc2d::event::EventError> {

        self.map.draw(gc2d);


        gc2d.graphics.print(format!("{}", dt), 0., 0., Some(Color::RED), fonts);

        Ok(())
    }

}
