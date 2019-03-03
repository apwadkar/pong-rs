use crate::pong::{Ball, MyEvent};
use amethyst::{
    core::{
        timing::{Stopwatch, Time},
        transform::Transform,
    },
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};

pub struct MoveBallsSystem {
    pub is_reset: bool,
    reader: Option<ReaderId<MyEvent>>,
    timer: Stopwatch,
}

impl Default for MoveBallsSystem {
    fn default() -> Self {
        MoveBallsSystem {
            is_reset: false,
            reader: None,
            timer: Stopwatch::new(),
        }
    }
}

impl<'s> System<'s> for MoveBallsSystem {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, EventChannel<MyEvent>>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader = Some(res.fetch_mut::<EventChannel<MyEvent>>().register_reader());
    }

    fn run(&mut self, (balls, mut locals, time, event_channel): Self::SystemData) {
        for event in event_channel.read(self.reader.as_mut().unwrap()) {
            match *event {
                MyEvent::ResetBall => {
                    self.is_reset = true;
                    self.timer.restart();
                }
            }
        }
        for (ball, local) in (&balls, &mut locals).join() {
            if !self.is_reset {
                local.translate_x(ball.velocity[0] * time.delta_seconds());
                local.translate_y(ball.velocity[1] * time.delta_seconds());
            }
        }
        if self.is_reset {
            if self.timer.elapsed().as_secs() == 1 {
                self.is_reset = false;
                self.timer.stop();
            }
        }
    }
}
