use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawParam, FillOptions, Mesh, MeshBuilder, Rect};
use ggez::graphics::DrawMode::Fill;
use ggez::mint::Point2;

pub struct DrawImageState {
    image: graphics::Image,
}

impl DrawImageState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        /// 使用该路径前，请手动将"spacefox_16x16.png"复制到
        /// 编译的生成的target/debug/resources目录下（没有请手动创建）
        let image = graphics::Image::from_path(ctx, "/spacefox_16x16.png")?;
        Ok(DrawImageState { image })
    }
}

impl EventHandler for DrawImageState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // 1. 构造canvas实例
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));

        // 2. 绘制图片到指定位置
        const TILE_SIZE: f32 = 16.;
        const IMG_W: f32 = 256.;
        const IMG_H: f32 = 256.;
        let src_rect = Rect::new(11. * TILE_SIZE, 0. * TILE_SIZE, TILE_SIZE, TILE_SIZE);
        // 比例转化后的矩形
        let ratio_src_rect = Rect::new(
            src_rect.x / IMG_W,
            src_rect.y / IMG_H,
            TILE_SIZE / IMG_W,
            TILE_SIZE / IMG_H,
        );
        canvas.draw(&self.image, DrawParam::new().src(ratio_src_rect).dest(Vec2::new(0.0, 0.0)));

        // 将整张图绘制到上个单图块右边
        canvas.draw(&self.image, DrawParam::new().dest(Vec2::new(TILE_SIZE + 10., TILE_SIZE + 10.)));
        // 3. finish
        canvas.finish(ctx)?;
        Ok(())
    }
}