use bevy::prelude::*;
use bevy_event_component::{CEvent, EventComponent, EventComponentRefExt};

#[derive(Component)]
struct MySprite;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        MySprite,
        Sprite {
            custom_size: Some(Vec2::splat(256.0)),
            color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        },
        Pickable::default(),
        EventComponent::<PointerClick>::default(),
    ));
}

fn update(query: Query<CEvent<PointerClick>, With<MySprite>>) {
    for event in query.iter() {
        if event.triggered_event().is_some() {
            println!("CLICK");
        }
    }
}
