use runty8::{App, Button};

struct Game {
    x: i32,
    y: i32,
}

// Your game implementation here:
impl App for Game {
    fn init(_pico8: &mut runty8::Pico8) -> Self {
        Self { x: 64, y: 64 }
    }

    fn update(&mut self, pico8: &mut runty8::Pico8) {
        if pico8.btn(Button::Left) {
            self.x -= 1;
        }
        if pico8.btn(Button::Right) {
            self.x += 1;
        }
        if pico8.btn(Button::Up) {
            self.y -= 1;
        }
        if pico8.btn(Button::Down) {
            self.y += 1;
        }
    }

    fn draw(&mut self, pico8: &mut runty8::Pico8) {
        pico8.cls(0);

        pico8.print("PRESS ARROW KEYS TO MOVE", 4, 4, 7);
        pico8.rectfill(
            self.x - Self::WIDTH / 2,
            self.y - Self::HEIGHT / 2,
            self.x + Self::WIDTH / 2,
            self.y + Self::HEIGHT / 2,
            7,
        );
        pico8.pset(self.x, self.y, 8);
    }
}

impl Game {
    const WIDTH: i32 = 7;
    const HEIGHT: i32 = 7;
}

fn main() {
    let assets = runty8::load_assets!("src").unwrap();

    runty8::debug_run::<Game>(assets).unwrap();
}
