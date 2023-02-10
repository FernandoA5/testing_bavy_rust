use bevy::prelude::*;
use heron::prelude::*;
use rand::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            title: "Pong".to_string(),
            width: 640.0,
            height: 480.0,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugin(PhysicsPlugin::default())
        .add_plugins(DefaultPlugins)
        .run();
}
