use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{
        Join,
        System,
        SystemData,
        World,
        WriteStorage,
    },
};

use crate::pong::{Ball, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, mut locals): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut locals).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                println!("player 2 score");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                println!("player 1 score");
                true
            } else {
                false
            };

            if did_hit {
                ball.velocity[0] = -ball.velocity[0];
                transform.set_translation_x(ARENA_WIDTH / 2.0);
                transform.set_translation_y(ARENA_HEIGHT / 2.0);
            }
        }
    }
}

