use bevy::prelude::*;
use bevy::input::gamepad::Gamepad;
use bevy::input::touch::*;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::window::CursorMoved;

pub fn mouse_click_system(
	mut commands: Commands,
	mouse_button_input: Res<Input<MouseButton>>,
) {
	if mouse_button_input.pressed(MouseButton::Left) {
		//info!("left mouse currently pressed");
	}

	if mouse_button_input.just_pressed(MouseButton::Left) {
	}

	if mouse_button_input.just_released(MouseButton::Left) {
	}
}

pub fn touch_system(touches: Res<Touches>) {
	for touch in touches.iter_just_pressed() {
		info!(
            "just pressed touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
	}

	for touch in touches.iter_just_released() {
		info!(
            "just released touch with id: {:?}, at: {:?}",
            touch.id(),
            touch.position()
        );
	}

	for touch in touches.iter_just_cancelled() {
		info!("cancelled touch with id: {:?}", touch.id());
	}

	// you can also iterate all current touches and retrieve their state like this:
	for touch in touches.iter() {
		info!("active touch: {:?}", touch);
		info!("  just_pressed: {}", touches.just_pressed(touch.id()));
	}
}

/// This system prints out all mouse events as they come in
pub fn input_event_system(
	mut mouse_button_input_events: EventReader<MouseButtonInput>,
	mut mouse_motion_events: EventReader<MouseMotion>,
	mut cursor_moved_events: EventReader<CursorMoved>,
	mut mouse_wheel_events: EventReader<MouseWheel>,
	mut touch_events: EventReader<TouchInput>,
) {
	for event in mouse_button_input_events.iter() {
	}

	for event in mouse_motion_events.iter() {
		// Has event.delta as vec2
	}

	for event in cursor_moved_events.iter() {
		// event.position as Vec2
		//dbg!("{}", &event);
	}

	for event in mouse_wheel_events.iter() {
	}

	for event in touch_events.iter() {
		//debug!("{:?}", event);
	}
}

pub fn gamepad_system(
	gamepads: Res<Gamepads>,
	button_inputs: Res<Input<GamepadButton>>,
	button_axes: Res<Axis<GamepadButton>>,
	axes: Res<Axis<GamepadAxis>>,
) {
	for gamepad in gamepads.iter().cloned() {
		if button_inputs.just_pressed(GamepadButton(gamepad, GamepadButtonType::South)) {
			info!("{:?} just pressed South", gamepad);
		} else if button_inputs.just_released(GamepadButton(gamepad, GamepadButtonType::South)) {
			info!("{:?} just released South", gamepad);
		}

		let right_trigger = button_axes
			.get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
			.unwrap();
		if right_trigger.abs() > 0.01 {
			info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
		}

		let left_stick_x = axes
			.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
			.unwrap();
		if left_stick_x.abs() > 0.01 {
			info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
		}
	}
}

// Not using KB yet.
pub fn keyboard_system(input: Res<Input<KeyCode>>) {
	if input.any_pressed([KeyCode::LShift, KeyCode::RShift]) {
		println!("one or both of the two shift keys are pressed");
	}
}
