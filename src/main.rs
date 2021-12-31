mod com;
mod ent;
mod sys;

use bevy::core::FixedTimestep;
use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: String::from("Snake"),
            width: 550.,
            height: 550.,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(sys::SnakeSegments::default())
        .insert_resource(sys::LastTailPosition::default())
        .add_event::<sys::GrowthEvent>()
        .add_event::<sys::GameOverEvent>()
        .add_startup_system(sys::setup.system())
        .add_startup_stage(
            "game_setup",
            SystemStage::single(sys::snake_spawner.system()),
        )
        .add_system(
            sys::input
                .system()
                .label(sys::SnakeState::Input)
                .before(sys::SnakeState::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.15))
                .with_system(
                    sys::snake_movement
                        .system()
                        .label(sys::SnakeState::Movement),
                )
                .with_system(
                    sys::snake_eating
                        .system()
                        .label(sys::SnakeState::Eating)
                        .after(sys::SnakeState::Movement),
                )
                .with_system(
                    sys::snake_growth
                        .system()
                        .label(sys::SnakeState::Growth)
                        .after(sys::SnakeState::Eating),
                ),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(sys::position_translation.system())
                .with_system(sys::size_scaling.system()),
        )
        .add_system(sys::game_over.system().after(sys::SnakeState::Movement))
        .add_plugins(DefaultPlugins)
        .run();
}
