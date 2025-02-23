use crate::cli;

use bevy::prelude::*;
use clap::Parser;

#[cfg(feature = "k8s")]
use super::{k8s_file, k8s_live};

use super::{integration_utils::IntegrationTimers, test_mode};

pub struct IntegrationPlugin;

impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        let generator_choise = cli::Cli::parse().room_generator;
        match generator_choise {
            #[cfg(feature = "k8s")]
            Some(cli::RoomGeneratorType::K8sLive) => app
                .insert_resource(IntegrationTimers {
                    // BUG! seems when a larger cluster with logs of objects is present the game
                    // freezes while we scrape the info
                    k8s: Timer::from_seconds(60.0, TimerMode::Repeating),
                })
                .add_plugins(k8s_live::K8sIntegrationPlugin),
            #[cfg(feature = "k8s")]
            None | Some(cli::RoomGeneratorType::K8sFile) => {
                app.add_plugins(k8s_file::K8sIntegrationPlugin)
            }
            _ => app.add_plugins(test_mode::TestModeIntegrationPlugin),
        };
    }
}
