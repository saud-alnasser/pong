pub mod prelude {
    pub use super::events::*;
    pub use super::plugins::*;
    pub use super::resource::*;
}

pub mod events {
    use bevy::prelude::*;

    #[derive(Event, Debug)]
    pub struct GoalEvent;
}

pub mod resource {
    use bevy::prelude::*;

    #[derive(Resource, Debug)]
    pub struct Scoreboard {
        pub player: u32,
        pub opponent: u32,
    }

    impl Default for Scoreboard {
        fn default() -> Self {
            Self {
                player: 0,
                opponent: 0,
            }
        }
    }
}

pub mod systems {
    use crate::prelude::*;
    use bevy::prelude::*;

    pub fn detect_goal(
        (ball, player, opponent): (
            Query<&Transform, With<Ball>>,
            Query<&Transform, With<Player>>,
            Query<&Transform, With<Opponent>>,
        ),
        mut on_goal: EventWriter<GoalEvent>,
        mut score: ResMut<Scoreboard>,
    ) {
        let ball = match ball.iter().next() {
            Some(ball) => ball,
            None => return,
        };

        let player = match player.iter().next() {
            Some(player) => player,
            None => return,
        };

        let opponent = match opponent.iter().next() {
            Some(opponent) => opponent,
            None => return,
        };

        let margin = 50.0;

        if ball.translation.x < player.translation.x - margin {
            score.opponent += 1;
            on_goal.send(GoalEvent);
        } else if ball.translation.x > opponent.translation.x + margin {
            score.player += 1;
            on_goal.send(GoalEvent);
        }
    }
}

pub mod plugins {
    use crate::prelude::*;
    use bevy::prelude::*;

    use super::systems::detect_goal;

    pub struct ScorePlugin;

    impl Plugin for ScorePlugin {
        fn build(&self, app: &mut App) {
            app.add_event::<GoalEvent>()
                .insert_resource(Scoreboard::default())
                .add_systems(FixedUpdate, detect_goal);
        }
    }
}
