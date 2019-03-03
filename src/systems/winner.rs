use crate::pong::{Ball, MyEvent, ScoreBoard, ScoreText, ARENA_WIDTH, BALL_VELOCITY_X};
use amethyst::{core::transform::Transform, ecs::prelude::*, shrev::EventChannel, ui::UiText};

pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        Option<Read<'s, ScoreText>>,
        Write<'s, EventChannel<MyEvent>>,
    );

    fn run(
        &mut self,
        (mut balls, mut locals, mut ui_text, mut scores, score_text, mut event_channel): Self::SystemData,
    ) {
        if let Some(score_text) = score_text {
            for (ball, transform) in (&mut balls, &mut locals).join() {
                let mut new_vel: f32 = 0.0;
                let ball_x = transform.translation().x;

                let did_hit = if ball_x <= ball.radius {
                    scores.score_right = (scores.score_right + 1).min(999);
                    new_vel = -BALL_VELOCITY_X;

                    if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                        text.text = scores.score_right.to_string();
                    }
                    true
                } else if ball_x >= ARENA_WIDTH - ball.radius {
                    scores.score_left = (scores.score_left + 1).min(999);
                    new_vel = BALL_VELOCITY_X;

                    if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                        text.text = scores.score_left.to_string();
                    }
                    true
                } else {
                    false
                };

                if did_hit {
                    event_channel.single_write(MyEvent::ResetBall);
                    ball.velocity[0] = new_vel;
                    ball.velocity[1] = -ball.velocity[1];
                    transform.set_x(ARENA_WIDTH / 2.0);
                    transform.set_y(ARENA_WIDTH / 2.0);
                }
            }
        }
    }
}
