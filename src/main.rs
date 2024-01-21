use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_world, greet_people))
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Buzz Lightyear".to_string())));
    commands.spawn((Person, Name("Mario Mario".to_string())));
    commands.spawn((Person, Name("Link".to_string())));
}

fn hello_world() {
    println!("Hello world!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}!", name.0);
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Link" {
            name.0 = "Zelda".to_string();
            break;
        }
    }
}
