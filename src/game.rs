
use std::collections::HashMap;

use gc2d::{event::EventLoop, color::Color};
use gc2d_games::tilemap::{TileMap, TypeTileMap, TileDescription, TileMapDetail};


const MAP_TILE_HEIGHT: usize = 70;
const MAP_TILE_WIDTH: usize = 70;

pub struct Game {
    map: TileMap<u32, MyTileDescription>,

    player: Player,
}
struct MyTileDescription {
    name: String,
}

impl Default for MyTileDescription {
    fn default() -> Self {
        Self { 
            name: String::new(),
        }
    }
}

impl TileDescription for MyTileDescription {}


struct Player {
    x: f32, 
    y: f32,
}

impl Game {
    pub fn new() -> Self {

        // Create the definition of each tile
        let tiles_definition = HashMap::from([
            (1, TileMapDetail { 
                type_tilemap: TypeTileMap::FromSimpleFile("assets/images/grassCenter.png".to_string()),
                description: Some(MyTileDescription { name: String::from("Grass")}),
            }),
            (2, TileMapDetail {
                type_tilemap: TypeTileMap::FromSimpleFile("assets/images/liquidLava.png".to_string()),
                description: Some(MyTileDescription { name: String::from("LiquidLava")}),
            }),
            (3, TileMapDetail {
                type_tilemap: TypeTileMap::FromSimpleFile("assets/images/liquidWater.png".to_string()),
                description: Some(MyTileDescription { name: String::from("LiquidWater")}),
            }),
        ]);

        Self {
            map: TileMap::new(
                tiles_definition,
                MAP_TILE_HEIGHT,
                MAP_TILE_WIDTH,
            ),
            player : Player {
                x: 0f32,
                y: 0f32,
            }
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

    fn update(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, dt: f32, _audio_manager: &mut gc2d::audio::AudioManager) -> Result<(), gc2d::event::EventError> {
        if gc2d.keyboard.is_down(gc2d::keyboard::KeyCode::Left) {
            self.player.x -= 50. * dt;
        }
        if gc2d.keyboard.is_down(gc2d::keyboard::KeyCode::Right) {
            self.player.x += 50. * dt;
        }
        if gc2d.keyboard.is_down(gc2d::keyboard::KeyCode::Down) {
            self.player.y += 50. * dt;
        }
        if gc2d.keyboard.is_down(gc2d::keyboard::KeyCode::Up) {
            self.player.y -= 50. * dt;
        }
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, fonts: &mut gc2d::fonts::FontsManager, _dt: f32) -> Result<(), gc2d::event::EventError> {

        self.map.draw(gc2d);

        // Display informations of tile at position x, y of the mouse
        if let Some(map_description) = self.map.get_tile_at_position(gc2d.mouse.x, gc2d.mouse.y) {
            gc2d.graphics.print(
                format!("Mouse X: {}, Mouse Y: {} : {}", gc2d.mouse.x, gc2d.mouse.y, map_description.name.clone()), 
                10., 10., 
                Some(Color::BLACK), 
                fonts
            );
        }

        // Draw player
        gc2d.graphics.circle(gc2d::graphics::DrawMode::Fill, self.player.x + 10., self.player.y + 10., 10., Some(Color::RED));

        Ok(())
    }

}
