mod draw_shape;
mod draw_house;
mod draw_multi_house;
mod draw_image;
mod draw_text;

use ggez::{event, GameError};
use ggez::conf::WindowMode;
use ggez::event::EventHandler;
/// 相关State
use crate::draw_multi_house::DrawMultiHouseState;
use crate::draw_house::DrawHouseState;
use crate::draw_image::DrawImageState;
use crate::draw_shape::DrawShapeState;
use crate::draw_text::DrawTextState;


fn main() -> Result<(), GameError> {
    let cb = ggez::ContextBuilder::new("drawing", "w4ngzhen")
        .window_mode(WindowMode::default().dimensions(1600f32, 1200f32));
    let (mut ctx, events_loop) = cb.build()?;
    // 使用上面的DrawXXXState，查看各种效果。
    let state = DrawShapeState::new(&mut ctx)?;
    event::run(ctx, events_loop, state)
}