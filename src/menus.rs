use bevy::prelude::*;
use bevy_quickmenu::{
    style::Stylesheet, ActionTrait, Menu, MenuIcon, MenuItem, MenuOptions, MenuState,
    QuickMenuPlugin, ScreenTrait,
};

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        let options = MenuOptions {
            font: Some("fonts/Alexandria.ttf"),
            ..Default::default()
        };
        app.add_event::<BasicMenuEvent>()
            .add_startup_system(setup_menu)
            .add_system(event_reader)
            .add_plugin(QuickMenuPlugin::<Screens>::with_options(options));
    }
}

fn setup_menu(mut commands: Commands) {
    let sheet = Stylesheet::default();

    commands.insert_resource(MenuState::new(
        BasicMenuState::default(),
        Screens::Root,
        Some(sheet),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Screens {
    Root,
    Bools,
}

impl ScreenTrait for Screens {
    type Action = BasicMenuActions;
    type State = BasicMenuState;
    fn resolve(&self, state: &<<Self as ScreenTrait>::Action as ActionTrait>::State) -> Menu<Self> {
        match self {
            Screens::Root => root_menu(state),
            Screens::Bools => bools_menu(state),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum BasicMenuActions {
    Close,
    Toggle1,
    Toggle2,
}

impl ActionTrait for BasicMenuActions {
    type State = BasicMenuState;
    type Event = BasicMenuEvent;
    fn handle(&self, state: &mut BasicMenuState, event_writer: &mut EventWriter<BasicMenuEvent>) {
        match self {
            BasicMenuActions::Close => event_writer.send(BasicMenuEvent::Close),
            BasicMenuActions::Toggle1 => state.bool1 = !state.bool1,
            BasicMenuActions::Toggle2 => state.bool2 = !state.bool2,
        }
    }
}

#[derive(Debug)]
enum BasicMenuEvent {
    Close,
}

#[derive(Debug, Clone, Default)]
struct BasicMenuState {
    bool1: bool,
    bool2: bool,
}

fn root_menu(_state: &BasicMenuState) -> Menu<Screens> {
    Menu::new(
        "root",
        vec![
            MenuItem::headline("Basic Example"),
            MenuItem::action("Close", BasicMenuActions::Close).with_icon(MenuIcon::Back),
            MenuItem::label("A Submenu"),
            MenuItem::screen("Boolean", Screens::Bools),
        ],
    )
}

fn bools_menu(state: &BasicMenuState) -> Menu<Screens> {
    Menu::new(
        "boolean",
        vec![
            MenuItem::label("Toggles some booleans"),
            MenuItem::action("Toogle Bool1", BasicMenuActions::Toggle1).checked(state.bool1),
            MenuItem::action("Toggle Bool2", BasicMenuActions::Toggle2).checked(state.bool2),
        ],
    )
}

fn event_reader(mut commands: Commands, mut event_reader: EventReader<BasicMenuEvent>) {
    for event in event_reader.iter() {
        match event {
            BasicMenuEvent::Close => bevy_quickmenu::cleanup(&mut commands),
        }
    }
}
