use bevy::prelude::{App, Component, Update};

fn main() {
    App::new().add_systems(Update, hello_world).run();
}

fn hello_world() {
    println!("hello world");
}
