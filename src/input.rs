use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default)]
pub enum PlayerAction {
    #[default]
    Pan,
    PanGamepad,
    Jump,
    Move,
    SwitchPerspective,
    Shoot,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    #[bundle]
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map =
            input_map::InputMap::new([(KeyCode::Space, Jump), (KeyCode::Q, SwitchPerspective)])
                .insert(MouseButton::Left, Shoot)
                .insert_multiple([
                    (DualAxis::mouse_motion(), Pan),
                    (DualAxis::right_stick(), PanGamepad),
                    (DualAxis::left_stick(), Move),
                ])
                .insert(VirtualDPad::wasd(), Move)
                .set_gamepad(Gamepad { id: 1 })
                .build();

        InputListenerBundle {
            input_manager: InputManagerBundle {
                input_map,
                ..Default::default()
            },
        }
    }
}
