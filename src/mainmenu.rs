use crate::pong::{initialize_camera, Pong};
use amethyst::{
    assets::Loader,
    input::is_key_down,
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{Anchor, TtfFormat, UiButtonBuilder, UiEventType},
};

fn initialize_menu(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    UiButtonBuilder::new("play_button".to_string(), "Play Co-op".to_string())
        .with_font(font.clone())
        .with_font_size(30.)
        .with_text_color([1., 1., 1., 1.])
        .with_hover_text_color([1., 0., 0., 1.])
        .with_anchor(Anchor::Middle)
        .with_position(0., 0.)
        .with_layer(1.)
        .with_size(150., 50.)
        .build_from_world(world);
    
    UiButtonBuilder::new("playai_button".to_string(), "Play AI".to_string())
        .with_font(font.clone())
        .with_font_size(30.)
        .with_text_color([1., 1., 1., 1.])
        .with_hover_text_color([1., 0., 0., 1.])
        .with_anchor(Anchor::TopMiddle)
        .with_position(0., 0.)
        .with_layer(1.)
        .with_size(150., 50.)
        .build_from_world(world);

    UiButtonBuilder::new("playnet_button".to_string(), "Play Online".to_string())
        .with_font(font)
        .with_font_size(30.)
        .with_text_color([1., 1., 1., 1.])
        .with_hover_text_color([1., 0., 0., 1.])
        .with_anchor(Anchor::BottomMiddle)
        .with_position(0., 0.)
        .with_layer(1.)
        .with_size(150., 50.)
        .build_from_world(world);
}

pub struct MainMenu;

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        initialize_camera(world);
        initialize_menu(world);
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match &event {
            StateEvent::Window(wnd_event) => {
                if is_key_down(&wnd_event, VirtualKeyCode::P) {
                    Trans::Switch(Box::new(Pong))
                } else if is_key_down(&wnd_event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                if ui_event.event_type == UiEventType::Click {
                    
                    Trans::Switch(Box::new(Pong))
                } else {
                    Trans::None
                }
            }
        }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
    }
}
