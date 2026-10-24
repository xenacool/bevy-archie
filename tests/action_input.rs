use bevy::input::keyboard::{Key, KeyCode, KeyboardInput, NativeKey};
use bevy::input::gamepad::{
    GamepadButton, GamepadConnection, GamepadConnectionEvent, RawGamepadButtonChangedEvent,
    RawGamepadEvent,
};
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::input::{ButtonState, InputPlugin};
use bevy::prelude::*;
use bevy_archie::actions::{update_action_state, ActionMap, ActionState, VirtualGamepadInput};
use bevy_archie::testing::{ActionWaitFor, update_until_action};
use bevy_archie::touch_joystick::{TouchJoystick, TouchJoystickInputSet, TouchJoystickPlugin};

fn input_app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(InputPlugin)
        .add_plugins(TouchJoystickPlugin)
        .init_resource::<ActionMap>()
        .init_resource::<ActionState>()
        .init_resource::<VirtualGamepadInput>()
        .add_systems(
            PreUpdate,
            update_action_state.after(TouchJoystickInputSet),
        );
    let window = app.world_mut().spawn(Window::default()).id();
    (app, window)
}

#[test]
fn keyboard_event_reaches_archie_action_before_test_assertion() {
    let (mut app, window) = input_app();
    app.world_mut().write_message(KeyboardInput {
        key_code: KeyCode::Enter,
        logical_key: Key::Unidentified(NativeKey::Android(66)),
        state: ButtonState::Pressed,
        text: None,
        repeat: false,
        window,
    });

    let updates = update_until_action(
        &mut app,
        bevy_archie::actions::GameAction::Confirm,
        ActionWaitFor::JustPressed,
        2,
    )
        .expect("keyboard input should become Confirm before the bounded wait expires");

    assert_eq!(updates, 1);
}

#[test]
fn mouse_button_event_reaches_archie_action_before_test_assertion() {
    let (mut app, window) = input_app();
    app.world_mut().write_message(MouseButtonInput {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
        window,
    });

    update_until_action(
        &mut app,
        bevy_archie::actions::GameAction::Confirm,
        ActionWaitFor::JustPressed,
        2,
    )
        .expect("mouse input should become Confirm before the bounded wait expires");
}

#[test]
fn virtual_gamepad_button_reaches_archie_action_before_test_assertion() {
    let (mut app, _) = input_app();
    let sequence = app.world_mut()
        .resource_mut::<VirtualGamepadInput>()
        .set_button(GamepadButton::South, true);

    update_until_action(
        &mut app,
        bevy_archie::actions::GameAction::Confirm,
        ActionWaitFor::JustPressed,
        2,
    )
        .expect("virtual gamepad input should become Confirm before the bounded wait expires");
    assert!(bevy_archie::virtual_input::virtual_input_acknowledged() >= sequence);
}

#[test]
fn physical_gamepad_event_reaches_archie_action_before_test_assertion() {
    let (mut app, _) = input_app();
    let gamepad = app.world_mut().spawn_empty().id();
    app.world_mut().write_message(GamepadConnectionEvent::new(
        gamepad,
        GamepadConnection::Connected {
            name: "Archie integration test pad".into(),
            vendor_id: None,
            product_id: None,
        },
    ));
    app.world_mut().write_message(RawGamepadEvent::Button(
        RawGamepadButtonChangedEvent::new(gamepad, GamepadButton::South, 1.0),
    ));

    update_until_action(
        &mut app,
        bevy_archie::actions::GameAction::Confirm,
        ActionWaitFor::JustPressed,
        2,
    )
        .expect("physical gamepad event should become Confirm before the bounded wait expires");
}

#[test]
fn touch_joystick_movement_reaches_archie_action_before_test_assertion() {
    let (mut app, window) = input_app();
    app.world_mut().spawn(
        TouchJoystick::fixed(Vec2::new(100.0, 100.0)).with_radius(50.0),
    );
    app.world_mut().write_message(TouchInput {
        phase: TouchPhase::Started,
        position: Vec2::new(100.0, 100.0),
        window,
        force: None,
        id: 7,
    });
    app.update();
    app.world_mut().write_message(TouchInput {
        phase: TouchPhase::Moved,
        position: Vec2::new(140.0, 100.0),
        window,
        force: None,
        id: 7,
    });

    update_until_action(
        &mut app,
        bevy_archie::actions::GameAction::Right,
        ActionWaitFor::JustPressed,
        2,
    )
        .expect("touch joystick input should become Right before the bounded wait expires");
}