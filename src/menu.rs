use bevy::{prelude::*, utils::info};
use bevy_quinnet::server;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(ClientMode::Client).add_systems(Startup, setup).add_systems(Update, button_system);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const SELECTED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const PRESSED_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.35, 0.35, 0.35);


fn setup(mut commands: Commands, client_mode: Res<State<ClientMode>>){
    let text = "Choose your mode";

    commands.spawn(
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ) // Set the alignment of the Text
        .with_text_justify(JustifyText::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Auto,
            right: Val::Auto,
            justify_self: JustifySelf::Center,
            ..default()
        })
    );



    let container_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let client_button_node =  ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    let client_button_text_node = TextBundle::from_section(
        "Client",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let server_button_node =  ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..default()
    };

    let server_button_text_node = TextBundle::from_section(
        "Server",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let container = commands.spawn(container_node).id();

    let client_button = commands.spawn((client_button_node,ModeButtonType(ClientMode::Client))).id();
    let client_button_text = commands.spawn(client_button_text_node).id();

    let server_button = commands.spawn((server_button_node,ModeButtonType(ClientMode::Server))).id();
    let server_button_text = commands.spawn(server_button_text_node).id();

    commands.entity(client_button).push_children(&[client_button_text]);
    commands.entity(server_button).push_children(&[server_button_text]);

    commands.entity(container).push_children(&[client_button]).push_children(&[server_button]);
}


#[derive(Component)]
struct ModeButtonType(ClientMode);
fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ModeButtonType,
        ),
        (
            // Changed<Interaction>,
            With<Button>,
        ),
    >,
    mut text_query: Query<&mut Text>,
    client_mode: Res<State<ClientMode>>,
    mut client_mode_next: ResMut<NextState<ClientMode>>,
) {
    for (
        interaction,
        mut color,
        mut border_color,
        children,
        button_type,
    ) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                client_mode_next.set(button_type.0.clone());
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                if button_type.0 == *client_mode.get(){
                    *color = SELECTED_BUTTON.into();
                }else{
                    *color = NORMAL_BUTTON.into();
                }
                border_color.0 = Color::BLACK;
            }
        }
    }
}





#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum ClientMode {
    Server,
    Client
}