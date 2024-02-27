use ggez::{Context, ContextBuilder, GameError, GameResult};
use ggez::graphics::{self, Color, DrawParam, Quad};
use ggez::event::{self, EventHandler};
use ggez::mint::Point2;
use ggegui::{egui, Gui};
use ggez::glam::Vec2;
use ggez::input::keyboard::KeyInput;
use ggez::winit::event::VirtualKeyCode;
use specs::{Builder, Component, Join, ReadStorage, RunNow, System, VecStorage, World, WorldExt};

// 创建两个Component
struct Player {}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

struct Name {
    name: String,
}

impl Component for Name {
    type Storage = VecStorage<Self>;
}

// 创建系统
struct NamePrintSystem {}

impl<'a> System<'a> for NamePrintSystem {
    type SystemData = (ReadStorage<'a, Name>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        let (name_storage, play_storage) = data;
        for (name, _player) in (&name_storage, &play_storage).join() {
            println!("player has name: {:?}", name.name);
        }
    }
}


fn main() {
    let mut world = World::new();
    // 注册Component
    world.register::<Player>();
    world.register::<Name>();
    // 创建实体
    world.create_entity()
        .with(Player {})
        .with(Name { name: "Tom".to_string() })
        .build();

    let (ctx, event_loop) =
        ContextBuilder::new("my_game", "Cool Game Author")
            .build()
            .expect("error");
    let my_game = MyGame {
        x: 0,
        to_right: true,
        gui: Gui::new(&ctx),
        world,
    };
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    x: i32,
    to_right: bool,
    gui: Gui,
    world: World,
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
        // ggegui更新逻辑
        let gui_ctx = self.gui.ctx();
        egui::Window::new("Title").show(&gui_ctx, |ui| {
            ui.label("label");
            if ui.button("button").clicked() {
                println!("button clicked");
            }
        });
        self.gui.update(_ctx);
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
        // GUI绘制
        canvas.draw(
            &self.gui,
            DrawParam::default().dest(Vec2::ZERO),
        );
        // 提交绘图
        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        if let Some(keycode) = input.keycode {
            if keycode == VirtualKeyCode::Return {
                // 如果按下回车，则将“名字打印系统”执行一次
                let mut name_print_system = NamePrintSystem {};
                name_print_system.run_now(&self.world);
                // 并进行一次ECS系统的数据更新
                self.world.maintain();
            }
        }
        Ok(())
    }
}