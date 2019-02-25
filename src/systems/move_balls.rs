use amethyst::{
    core::{
        timing::Time,
        transform::Transform,
    },
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
    input::InputHandler,
};
use crate::pong::{Ball, WinEvent};

pub struct MoveBallsSystem {
    pub is_reset: bool,
    reader: Option<ReaderId<WinEvent>>,
}

impl Default for MoveBallsSystem {
    fn default() -> Self {
        MoveBallsSystem {
            is_reset: true,
            reader: None
        }
    }
}

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, EventChannel<WinEvent>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<WinEvent>>().register_reader());
    }

    fn run(&mut self, (balls, mut locals, time, input, event_channel): Self::SystemData) {
        for event in event_channel.read(self.reader.as_mut().unwrap()) {
            if *event == WinEvent::ResetBall {
                self.is_reset = true;
            }
        }
        for (ball, local) in (&balls, &mut locals).join() {
            if !self.is_reset {
                local.translate_x(ball.velocity[0] * time.delta_seconds());
                local.translate_y(ball.velocity[1] * time.delta_seconds());
            }
        }
        if let Some(reset) = input.action_is_down("start_round") {
            if reset {
                self.is_reset = false;
            }
        }
    }
}
