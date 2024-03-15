use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, FillOptions, Mesh, MeshBuilder, Rect};
use ggez::graphics::DrawMode::Fill;
use ggez::mint::Point2;

pub struct DrawHouseState {
    house: Mesh,
}

impl DrawHouseState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut builder = MeshBuilder::new();
        let house_mesh_data = builder
            .polygon(Fill(FillOptions::default()),
                     &[[0., 0.], [-50., 50.], [50., 50.]],
                     Color::from_rgb(165, 106, 54))?
            .rectangle(Fill(FillOptions::default()),
                       Rect::new(-40., 50., 80., 50.),
                       Color::from_rgb(246, 236, 195))?
            .rectangle(Fill(FillOptions::default()),
                       Rect::new(0.0, 70., 20., 30.),
                       Color::from_rgb(165, 106, 54))?
            .build();
        let house = Mesh::from_data(ctx, house_mesh_data);
        Ok(DrawHouseState {
            house
        })
    }
}

impl EventHandler for DrawHouseState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // 1. 构造canvas实例
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        // 2. DrawParam和绘制一次
        let draw_param = DrawParam::default()
            .dest(Point2::from([100., 100.]));
        canvas.draw(&self.house, draw_param.clone());

        // 3. finish
        canvas.finish(ctx)?;
        Ok(())
    }
}