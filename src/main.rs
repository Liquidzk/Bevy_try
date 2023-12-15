// use bevy::prelude::*;

// pub fn main() {
//     App::new().add_systems(Update, hello_world_system).run();
// }

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world_system, greet_people))
        .run();
}

fn add_people(mut command: Commands) {
    command.spawn((Person, Name("Elaina Proctor".to_string())));
}

fn hello_world_system() {
    println!("hello world");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}
