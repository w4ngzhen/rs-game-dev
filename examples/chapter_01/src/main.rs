use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, DrawParam, Quad};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;

fn main() {
    let (ctx, event_loop) =
        ContextBuilder::new("my_game", "Cool Game Author")
            .build()
            .expect("error");
    let my_game = MyGame {
        x: 0,
        to_right: true
    };
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    x: i32,
    to_right: bool,
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // 方块位置逻辑更新
        let width = _ctx.gfx.window().inner_size().width as i32;
        if self.to_right {
            let next_x = self.x + 5;
            if next_x > width {
                self.to_right = false;
                self.x = width;
            } else {
                self.to_right = true;
                self.x = next_x;
            }
        } else {
            let next_x = self.x - 5;
            if next_x < 0 {
                self.to_right = true;
                self.x = 0;
            } else {
                self.to_right = false;
                self.x = next_x;
            }
        }
        // OK
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // 自己红色方块的绘制
        let draw_param = DrawParam::new()
            .dest(Point2::from([self.x as f32, 10.0]))
            .scale(Point2::from([20.0, 20.0]))
            .color(Color::from_rgb(0xFF, 0, 0));
        canvas.draw(&Quad, draw_param);
        // 提交绘图
        canvas.finish(ctx)
    }
}