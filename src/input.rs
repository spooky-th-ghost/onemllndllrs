use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, *};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Default, Reflect)]
pub enum PlayerAction {
    #[default]
    Pan,
    PanGamepad,
    Jump,
    Move,
    SwitchPerspective,
    Shoot,
    AimDownSights,
}

#[derive(Bundle)]
pub struct InputListenerBundle {
    input_manager: InputManagerBundle<PlayerAction>,
}

impl InputListenerBundle {
    pub fn input_map() -> InputListenerBundle {
        use PlayerAction::*;

        let input_map = input_map::InputMap::new([
            (KeyCode::Space, Jump),
            (KeyCode::Q, SwitchPerspective),
            (KeyCode::ShiftLeft, AimDownSights),
        ])
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
