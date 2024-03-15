use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, FillOptions, Mesh};
use ggez::graphics::DrawMode::Fill;
use ggez::mint::Point2;

pub struct DrawShapeState {
    x: u32,
    red: f32,
    shape_mesh: Mesh,
}

impl DrawShapeState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let shape = Mesh::new_circle(ctx,
                                     Fill(FillOptions::default()),
                                     [50., 50.],
                                     25.,
                                     0.01,
                                     Color::WHITE)?;
        Ok(DrawShapeState {
            x: 0,
            red: 0.,
            shape_mesh: shape,
        })
    }
}

impl EventHandler for DrawShapeState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut curr_x = self.x;
        let window_w = ctx.gfx.window().inner_size().width;
        let next_x = if curr_x + 5 > window_w {
            0
        } else {
            curr_x + 5
        };
        self.x = next_x;
        self.red = next_x as f32 / window_w as f32;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // 1. 构造canvas实例
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        // 2. 绘图
        // 2.1 Mesh数据（无需构建，利用已有）
        let shape_mesh = &self.shape_mesh;
        // 2.2 构造当前位置的DrawParam
        let draw_param = DrawParam::default()
            .dest(Point2::from([self.x as f32, 100.]))
            .scale(Point2::from([1., 1.]))
            .color(Color::new(self.red, 0.0, 0.0, 1.0));
        // 2.3 绘制
        canvas.draw(shape_mesh, draw_param);

        // 3. finish
        canvas.finish(ctx)?;
        Ok(())
    }
}