use bevy::{audio::Volume, prelude::*};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sounds);
    }
}

#[derive(Resource)]
pub struct SoundBank {
    pub gun_shot: Handle<AudioSource>,
    pub gun_empty: Handle<AudioSource>,
}

impl SoundBank {
    pub fn bullet_shot(&self) -> AudioBundle {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let speed = rng.gen_range(1.0..3.5);
        let volume = rng.gen_range(0.8..1.1);

        AudioSourceBundle {
            source: self.gun_shot.clone(),
            settings: PlaybackSettings::DESPAWN
                .with_volume(Volume::new_relative(volume))
                .with_speed(speed),
        }
    }

    pub fn empty_fire(&self) -> AudioBundle {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let speed = rng.gen_range(1.0..1.5);
        let volume = rng.gen_range(0.8..1.1);

        AudioSourceBundle {
            source: self.gun_empty.clone(),
            settings: PlaybackSettings::DESPAWN
                .with_volume(Volume::new_relative(volume))
                .with_speed(speed),
        }
    }
}

pub fn load_sounds(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(SoundBank {
        gun_shot: assets.load("gunshot.ogg"),
        gun_empty: assets.load("gun_empty.ogg"),
    });
}
