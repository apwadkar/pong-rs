use crate::pong::{Ball, WinEvent};
use amethyst::{
    core::{
        timing::{Stopwatch, Time},
        transform::Transform,
    },
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};
use std::time::Duration;

pub struct MoveBallsSystem {
    pub is_reset: bool,
    reader: Option<ReaderId<WinEvent>>,
    timer: Stopwatch,
}

impl Default for MoveBallsSystem {
    fn default() -> Self {
        MoveBallsSystem {
            is_reset: true,
            reader: None,
            timer: Stopwatch::new(),
        }
    }
}

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Option<Read<'s, Time>>,
        Option<Write<'s, EventChannel<WinEvent>>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<WinEvent>>().register_reader());
    }

    fn run(&mut self, (balls, mut locals, time, event_channel): Self::SystemData) {
        if let (Some(time), Some(event_channel)) = (time, event_channel) {
            for event in event_channel.read(self.reader.as_mut().unwrap()) {
                // match *event {
                //     WinEvent::ResetBall => {
                //         self.is_reset = true;
                //         self.timer.start();
                //     }
                // }
                println!("Received an event: {:?}", event);
            }
            for (ball, local) in (&balls, &mut locals).join() {
                if !self.is_reset {
                    local.translate_x(ball.velocity[0] * time.delta_seconds());
                    local.translate_y(ball.velocity[1] * time.delta_seconds());
                }
            }
            // if self.timer.elapsed() == Duration::from_secs(1) {
            //     self.is_reset = false;
            //     self.timer.stop();
            //     self.timer.reset();
            // }
        }
    }
}
