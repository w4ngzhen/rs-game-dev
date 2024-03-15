use ggez::{Context, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::graphics::{Color, DrawParam, Text, TextFragment};

pub struct DrawTextState {}

impl DrawTextState {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(DrawTextState {})
    }
}

impl EventHandler for DrawTextState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // 1. 构造canvas实例
        let mut canvas =
            graphics::Canvas::from_frame(ctx, Color::WHITE);

        // 2. 绘图
        // 2.1 简单绘制文本
        let simple_text = Text::new("hello, world.");
        canvas.draw(&simple_text, DrawParam::default().color(Color::BLACK));
        // 2.2 绘制多个文本片段
        let tf1 = TextFragment::new("RED").color(Color::RED);
        let tf2 = TextFragment::new("BLUE").color(Color::BLUE);
        let mut color_text = Text::new(tf1);
        color_text.add(tf2);
        canvas.draw(&color_text, DrawParam::default().dest([10., 20.]));

        // 3. finish
        canvas.finish(ctx)?;
        Ok(())
    }
}