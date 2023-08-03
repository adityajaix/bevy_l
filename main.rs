// // very basic
use bevy::prelude::*;

// // fn main() {
    // //     App::new().run();
    // // }
    // //---------------------------------------------------------------------------------
    // // ecs
    // fn hello_world() {
    //     println!("hello world!");
    // }
    

// // fn main() {
// //     App::new()
// //         .add_systems(Update, hello_world)
// //         .run();
// // }
// //---------------------------------------------------------
// first component
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// commands.spawn to spawn some entities.
// rust fn is system.
// I think following code is spawning 3 entities with 2 component person and name. 
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
// this is a system funtion, don't ask how
// parameter we passed query: Quary<component1, 2> will runs on all the entities with these components
// it "iterate over every Name component for entities that also have a Person component"

// fn greet_people(query: Query<&Name, With<Person>>) {
//     for name in &query {
//         println!("hello {}!", name.0);
//     }
    
// }

// fn main() {
//     App::new()
//         // add_systems takes 2 args, first to how frequent to call a fn and another is a fn or tuple of fn
//         // can have many add_systems for correct order
//         // here calling add_people fn
//         .add_systems(Startup, add_people)
//         // calling 2 fn h_w and g_p both will run in parrallel if possible
//         .add_systems(Update, (hello_world, greet_people))
//         .run();
// }

// -----------------------------------------------
// Plugins
// added default plugins
// created a custom plugins HelloPlugin to store systems
// for organization we use plugins need to implement plugin
// plugins can contain systems(rust fn) so we can group fns and add single custom plugin to add multiple systems 
// not maybe use pub if different file only
// pub struct HelloPlugin;

// // this takes Apps var arg. Which will be the App::new() var we created in the main fn
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Startup, add_people)
//             .add_systems(Update, (hello_world, greet_people));
//         // add things to your app here
//     }
// }
// fn main() {
//     App::new()
//         // adding defaults bevy plugins
//         // also this will open a window
//         // defaultplugin is similar to (
//         //     CorePlugin::default(),
//         //     InputPlugin::default(),
//         //     WindowPlugin::default(),
//         //     /* and many more more plugins omitted for brevity */
//         // )
//         // registering HelloPlugin for managing growing projects
//         .add_plugins((DefaultPlugins, HelloPlugin))
//         // add_systems takes 2 args, first to how frequent to call a fn and another is a fn or tuple of fn
//         // can have many add_systems for correct order
//         // here calling add_people fn
//         .add_systems(Startup, add_people)
//         // calling 2 fn h_w and g_p both will run in parrallel if possible
//         .add_systems(Update, (hello_world, greet_people))
//         .run();
// }

//-------------------------------------------------
// Resources
// resources is used for globally unique data
// ex 
// Elapsed Time
// Asset Collections (sounds, textures, meshes)
// Renderers

// creating a Resource by derive resource.
// Created a struct with Timer. Timer type is provided by bevy to track elapsed time
#[derive(Resource)]
struct GreetTimer(Timer);

// resources are accessed same way as component. means like filter.
// Res<Time> is in scope by adding DefaultPlugin in main fn.
// Res and ResMut pointers provide read and write access (respectively) to resources.
// delta field on Time gives us the time that has passed since the last update

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    // timer.0 is a timer field from GreetTimer struct
    // tick advance the timer by time passed.
    // we pass time of one frame. time.delta() gives time passed since last update. means 1 frame to another frame time
    // just_finished() return true when timer completes it's duration
    if timer.0.tick(time.delta()).just_finished(){
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // TimerMode could be Once or Repeating
        // inserting resource. don't know why it is called inser_resource rather than add_resource.
        // then just adding GreetTimer resouce with the Timer field and Setting timer value
        // from_seconds takes 2 args duration and mode.
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}