use gc2d::{event::EventLoop, color::Color};


const DEMO_FONT: &str = "assets/fonts/PixelMaster.ttf";
const DEMO_FONT_SIZE: u16 = 21;

pub struct Demo {
    rectangle_x: f32,
    rectangle_y: f32,
}

impl Default for Demo {
    fn default() -> Self {
        Demo {  
            rectangle_x: 50f32,
            rectangle_y: 100f32,
        }
    }
}

impl EventLoop for Demo {

    fn load(&mut self, gc2d: &mut gc2d::gc2d::Gc2d) -> Result<(), gc2d::event::EventError> {
        
        gc2d.window.set_title("Demo GC2D");
        gc2d.graphics.new_font(DEMO_FONT, DEMO_FONT_SIZE);

        Ok(())
    }

    fn update(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, dt: f32) -> Result<(), gc2d::event::EventError> {
        self.rectangle_x += 10f32 * dt;
        self.rectangle_y += 15f32 * dt;
        Ok(())
    }

    fn draw(&mut self, gc2d: &mut gc2d::gc2d::Gc2d, fonts: &mut gc2d::fonts::FontsManager) -> Result<(), gc2d::event::EventError> {
        
        gc2d.graphics.print(fonts, String::from("Demo GC2D"), 10f32, 10f32, Some(Color::BLUE));
        gc2d.graphics.rectangle(gc2d::graphics::DrawMode::Fill, self.rectangle_x, self.rectangle_y, 100f32, 75f32, Some(Color::RED));

        Ok(())    
    }

}