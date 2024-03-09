use ggez::conf::{WindowMode, WindowSetup};
use ggez::ContextBuilder;
use ggez::event::{self};
use pong::PongGame;


fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "Zach Kirlew")
        .window_setup(WindowSetup::default().title("Pong"))
        .window_mode(WindowMode::default().dimensions(1280f32, 800f32))
        .build()
        .expect("Zach Kirlew, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = PongGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

