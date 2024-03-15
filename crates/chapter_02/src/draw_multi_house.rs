use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, FillOptions, InstanceArray, Mesh, MeshBuilder, Rect};
use ggez::graphics::DrawMode::Fill;
use ggez::mint::Point2;

pub struct DrawMultiHouseState {
    house: Mesh,
}

impl DrawMultiHouseState {
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
        Ok(DrawMultiHouseState {
            house
        })
    }
}

impl EventHandler for DrawMultiHouseState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        println!("game fps: {:?}", _ctx.time.fps());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // 1. 构造canvas实例
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        // 400次绘制
        let mut instance_arr: InstanceArray = InstanceArray::new(ctx, None);
        for i in 0..20 {
            for j in 0..20 {
                const SIZE: f32 = 50.;
                let pos = Point2::from([i as f32 * SIZE, j as f32 * SIZE]);
                let scale = [SIZE / 100., SIZE / 100.];
                let draw_param = DrawParam::default().dest(pos).scale(scale);
                instance_arr.push(draw_param);
            }
        }
        canvas.draw_instanced_mesh(self.house.clone(), &instance_arr, DrawParam::default());

        // 3. finish
        canvas.finish(ctx)?;
        Ok(())
    }
}