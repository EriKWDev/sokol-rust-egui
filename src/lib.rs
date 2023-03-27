use sokol::app as sapp;
use sokol::gfx as sg;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Desc {
    pub gui_cb: Option<extern "C" fn(*const egui::Context)>,
    pub gui_userdata_cb: Option<extern "C" fn(*const egui::Context, *mut core::ffi::c_void)>,
    pub userdata: *mut core::ffi::c_void,
}

impl Desc {
    const fn new() -> Self {
        Self {
            gui_cb: None,
            gui_userdata_cb: None,
            userdata: core::ptr::null_mut(),
        }
    }
}

impl Default for Desc {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct State {
    pub context: egui::Context,
    pub raw_input: egui::RawInput,
    pub latest_output: Option<egui::FullOutput>,
    pub gui_cb: Option<extern "C" fn(*const egui::Context)>,
    pub gui_userdata_cb: Option<extern "C" fn(*const egui::Context, *mut core::ffi::c_void)>,
    pub userdata: *mut core::ffi::c_void,
}

static mut STATE: std::mem::MaybeUninit<State> = std::mem::MaybeUninit::uninit();

pub extern "C" fn frame() {
    let state = unsafe { &mut *STATE.as_mut_ptr() };

    let mut input = state.raw_input.clone();
    {
        input.screen_rect.replace(egui::Rect {
            max: egui::pos2(sapp::widthf(), sapp::heightf()),
            min: egui::pos2(0.0, 0.0),
        });
        input.pixels_per_point.replace(sapp::dpi_scale());
        input.predicted_dt = sapp::frame_duration() as _;
    }
    state.context.begin_frame(input);

    if let Some(cb) = state.gui_cb {
        (cb)(&state.context);
    }
    if let Some(cb) = state.gui_userdata_cb {
        (cb)(&state.context, state.userdata);
    }

    state.raw_input.events.clear();
    state.raw_input.events.clear();

    let output = state.context.end_frame();

    sapp::set_mouse_cursor(egui_cursor_to_sapp_cursor(
        output.platform_output.cursor_icon,
    ));

    // for event in &output.platform_output.events {
    //     match event {
    //         egui::output::OutputEvent::Clicked(_) => {}
    //         egui::output::OutputEvent::DoubleClicked(_) => {}
    //         egui::output::OutputEvent::TripleClicked(_) => {}
    //         egui::output::OutputEvent::FocusGained(_) => {}
    //         egui::output::OutputEvent::TextSelectionChanged(_) => {}
    //         egui::output::OutputEvent::ValueChanged(_) => {}
    //     }
    // }

    if let Some(request) = output.platform_output.open_url.as_ref() {
        /*
            TODO: Open the url
        */
        println!(
            "segui frame(): should open the url '{}' in new tab: {}",
            &request.url, request.new_tab
        );
    }

    if !output.platform_output.copied_text.is_empty() {
        /*
            TODO: Copy the text to clipboard
        */
        println!(
            "segui frame(): should send the text '{}' to system clipboard",
            &output.platform_output.copied_text
        );
    }

    state.latest_output = Some(output);
}

pub extern "C" fn draw() {
    let state = unsafe { &mut *STATE.as_mut_ptr() };

    if let Some(output) = state.latest_output.as_ref() {
        for (egui_texture_id, texture_delta) in &output.textures_delta.set {
            /*
                TODO: Update internal textures with the gived deltas
            */
            println!(
                "segui draw(): update texture with id '{egui_texture_id:?}' at '{:?}' with options '{:?}'",
                texture_delta.pos, texture_delta.options
            )
        }

        {
            /*
                TODO: Render gui using sokol::gfx
            */
            // println!("segui draw(): should draw the egui output using sokol::gfx",);
            // sg::apply_bindings(bindings);
            // sg::apply_pipeline(pip);
            // sg::draw(base_element, num_elements, num_instances);
        }

        for egui_texture_id in &output.textures_delta.free {
            /*
                TODO: Free internal textures with the given ids
            */
            println!("segui draw(): free texture with id '{egui_texture_id:?}'",)
        }

        if !output.repaint_after.is_zero() {
            /*
                TODO: Handle egui wait fo revents mode
            */
        }
    }
    /*
        TODO: else { something is wrong, log? }
    */
}

pub extern "C" fn begin(desc: &Desc) {
    /*
        TODO: Initialize required pipeline for rendering with gfx
    */

    let initial_state = State {
        context: egui::Context::default(),
        raw_input: egui::RawInput::default(),
        latest_output: None,

        gui_cb: desc.gui_cb,
        gui_userdata_cb: desc.gui_userdata_cb,
        userdata: desc.userdata,
    };

    unsafe {
        STATE.write(initial_state);
    }
}

pub extern "C" fn shutdown() {
    unsafe { STATE.assume_init_drop() }
}

#[rustfmt::skip]
pub fn egui_cursor_to_sapp_cursor(cursor: egui::CursorIcon) -> sapp::MouseCursor {
    match cursor {
        egui::CursorIcon::Default          => sapp::MouseCursor::Default,
        egui::CursorIcon::PointingHand     => sapp::MouseCursor::PointingHand,
        egui::CursorIcon::Crosshair        => sapp::MouseCursor::Crosshair,
        egui::CursorIcon::Text             => sapp::MouseCursor::Ibeam,
        egui::CursorIcon::VerticalText     => sapp::MouseCursor::Ibeam,
        egui::CursorIcon::NoDrop           => sapp::MouseCursor::NotAllowed,
        egui::CursorIcon::NotAllowed       => sapp::MouseCursor::NotAllowed,
        egui::CursorIcon::ResizeHorizontal => sapp::MouseCursor::ResizeEw,
        egui::CursorIcon::ResizeNeSw       => sapp::MouseCursor::ResizeNesw,
        egui::CursorIcon::ResizeNwSe       => sapp::MouseCursor::ResizeNwse,
        egui::CursorIcon::ResizeVertical   => sapp::MouseCursor::ResizeNs,
        egui::CursorIcon::ResizeEast       => sapp::MouseCursor::ResizeEw,
        egui::CursorIcon::ResizeSouthEast  => sapp::MouseCursor::ResizeAll,
        egui::CursorIcon::ResizeSouth      => sapp::MouseCursor::ResizeNs,
        egui::CursorIcon::ResizeSouthWest  => sapp::MouseCursor::ResizeAll,
        egui::CursorIcon::ResizeWest       => sapp::MouseCursor::ResizeEw,
        egui::CursorIcon::ResizeNorthWest  => sapp::MouseCursor::ResizeAll,
        egui::CursorIcon::ResizeNorth      => sapp::MouseCursor::ResizeNs,
        egui::CursorIcon::ResizeNorthEast  => sapp::MouseCursor::ResizeAll,

        egui::CursorIcon::ResizeColumn     => sapp::MouseCursor::ResizeNs, // TODO: Correct?
        egui::CursorIcon::ResizeRow        => sapp::MouseCursor::ResizeEw, // TODO: Correct? 

        // egui::CursorIcon::None             => sapp::MouseCursor::,
        // egui::CursorIcon::Help             => sapp::MouseCursor::,
        // egui::CursorIcon::ContextMenu      => sapp::MouseCursor::,
        // egui::CursorIcon::Progress         => sapp::MouseCursor::,
        // egui::CursorIcon::Wait             => sapp::MouseCursor::,
        // egui::CursorIcon::Cell             => sapp::MouseCursor::,
        // egui::CursorIcon::Alias            => sapp::MouseCursor::,
        // egui::CursorIcon::Copy             => sapp::MouseCursor::,
        // egui::CursorIcon::Move             => sapp::MouseCursor::,
        // egui::CursorIcon::Grab             => sapp::MouseCursor::,
        // egui::CursorIcon::Grabbing         => sapp::MouseCursor::,
        // egui::CursorIcon::AllScroll        => sapp::MouseCursor::,
        // egui::CursorIcon::ZoomIn           => sapp::MouseCursor::,
        // egui::CursorIcon::ZoomOut          => sapp::MouseCursor::,

        _ => sapp::MouseCursor::Default,
    }
}

#[rustfmt::skip]
pub fn sapp_mouse_button_to_egui_pointer_button(button: sapp::Mousebutton) -> egui::PointerButton {
    match button {
        sapp::Mousebutton::Left    => egui::PointerButton::Primary,
        sapp::Mousebutton::Right   => egui::PointerButton::Secondary,
        sapp::Mousebutton::Middle  => egui::PointerButton::Middle,
        sapp::Mousebutton::Invalid => egui::PointerButton::Extra1, // TODO: Valid?
    }
}

#[rustfmt::skip]
pub fn sapp_key_to_egui_key(keycode: sapp::Keycode) -> Option<egui::Key> {
    match keycode {
        sapp::Keycode::Space        => Some(egui::Key::Space),
        sapp::Keycode::Minus        => Some(egui::Key::Minus),
        sapp::Keycode::Num0         => Some(egui::Key::Num0),
        sapp::Keycode::Num1         => Some(egui::Key::Num1),
        sapp::Keycode::Num2         => Some(egui::Key::Num2),
        sapp::Keycode::Num3         => Some(egui::Key::Num3),
        sapp::Keycode::Num4         => Some(egui::Key::Num4),
        sapp::Keycode::Num5         => Some(egui::Key::Num5),
        sapp::Keycode::Num6         => Some(egui::Key::Num6),
        sapp::Keycode::Num7         => Some(egui::Key::Num7),
        sapp::Keycode::Num8         => Some(egui::Key::Num8),
        sapp::Keycode::Num9         => Some(egui::Key::Num9),
        sapp::Keycode::Equal        => Some(egui::Key::PlusEquals),
        sapp::Keycode::A            => Some(egui::Key::A),
        sapp::Keycode::B            => Some(egui::Key::B),
        sapp::Keycode::C            => Some(egui::Key::C),
        sapp::Keycode::D            => Some(egui::Key::D),
        sapp::Keycode::E            => Some(egui::Key::E),
        sapp::Keycode::F            => Some(egui::Key::F),
        sapp::Keycode::G            => Some(egui::Key::G),
        sapp::Keycode::H            => Some(egui::Key::H),
        sapp::Keycode::I            => Some(egui::Key::I),
        sapp::Keycode::J            => Some(egui::Key::J),
        sapp::Keycode::K            => Some(egui::Key::K),
        sapp::Keycode::L            => Some(egui::Key::L),
        sapp::Keycode::M            => Some(egui::Key::M),
        sapp::Keycode::N            => Some(egui::Key::N),
        sapp::Keycode::O            => Some(egui::Key::O),
        sapp::Keycode::P            => Some(egui::Key::P),
        sapp::Keycode::Q            => Some(egui::Key::Q),
        sapp::Keycode::R            => Some(egui::Key::R),
        sapp::Keycode::S            => Some(egui::Key::S),
        sapp::Keycode::T            => Some(egui::Key::T),
        sapp::Keycode::U            => Some(egui::Key::U),
        sapp::Keycode::V            => Some(egui::Key::V),
        sapp::Keycode::W            => Some(egui::Key::W),
        sapp::Keycode::X            => Some(egui::Key::X),
        sapp::Keycode::Y            => Some(egui::Key::Y),
        sapp::Keycode::Z            => Some(egui::Key::Z),
        sapp::Keycode::Escape       => Some(egui::Key::Escape),
        sapp::Keycode::Enter        => Some(egui::Key::Enter),
        sapp::Keycode::Tab          => Some(egui::Key::Tab),
        sapp::Keycode::Backspace    => Some(egui::Key::Backspace),
        sapp::Keycode::Insert       => Some(egui::Key::Insert),
        sapp::Keycode::Delete       => Some(egui::Key::Delete),
        sapp::Keycode::Right        => Some(egui::Key::ArrowRight),
        sapp::Keycode::Left         => Some(egui::Key::ArrowLeft),
        sapp::Keycode::Down         => Some(egui::Key::ArrowDown),
        sapp::Keycode::Up           => Some(egui::Key::ArrowUp),
        sapp::Keycode::PageUp       => Some(egui::Key::PageUp),
        sapp::Keycode::PageDown     => Some(egui::Key::PageDown),
        sapp::Keycode::Home         => Some(egui::Key::Home),
        sapp::Keycode::End          => Some(egui::Key::End),
        sapp::Keycode::F1           => Some(egui::Key::F1),
        sapp::Keycode::F2           => Some(egui::Key::F2),
        sapp::Keycode::F3           => Some(egui::Key::F3),
        sapp::Keycode::F4           => Some(egui::Key::F4),
        sapp::Keycode::F5           => Some(egui::Key::F5),
        sapp::Keycode::F6           => Some(egui::Key::F6),
        sapp::Keycode::F7           => Some(egui::Key::F7),
        sapp::Keycode::F8           => Some(egui::Key::F8),
        sapp::Keycode::F9           => Some(egui::Key::F9),
        sapp::Keycode::F10          => Some(egui::Key::F10),
        sapp::Keycode::F11          => Some(egui::Key::F11),
        sapp::Keycode::F12          => Some(egui::Key::F12),
        sapp::Keycode::F13          => Some(egui::Key::F13),
        sapp::Keycode::F14          => Some(egui::Key::F14),
        sapp::Keycode::F15          => Some(egui::Key::F15),
        sapp::Keycode::F16          => Some(egui::Key::F16),
        sapp::Keycode::F17          => Some(egui::Key::F17),
        sapp::Keycode::F18          => Some(egui::Key::F18),
        sapp::Keycode::F19          => Some(egui::Key::F19),
        sapp::Keycode::F20          => Some(egui::Key::F20),
        sapp::Keycode::Kp0          => Some(egui::Key::Num0),
        sapp::Keycode::Kp1          => Some(egui::Key::Num1),
        sapp::Keycode::Kp2          => Some(egui::Key::Num2),
        sapp::Keycode::Kp3          => Some(egui::Key::Num3),
        sapp::Keycode::Kp4          => Some(egui::Key::Num4),
        sapp::Keycode::Kp5          => Some(egui::Key::Num5),
        sapp::Keycode::Kp6          => Some(egui::Key::Num6),
        sapp::Keycode::Kp7          => Some(egui::Key::Num7),
        sapp::Keycode::Kp8          => Some(egui::Key::Num8),
        sapp::Keycode::Kp9          => Some(egui::Key::Num9),
        sapp::Keycode::KpSubtract   => Some(egui::Key::Minus),
        sapp::Keycode::KpAdd        => Some(egui::Key::PlusEquals),
        sapp::Keycode::KpEnter      => Some(egui::Key::Enter),
        sapp::Keycode::KpEqual      => Some(egui::Key::PlusEquals),

        // sapp::Keycode::F21          => Some(egui::Key::F21),
        // sapp::Keycode::F22          => Some(egui::Key::F22),
        // sapp::Keycode::F23          => Some(egui::Key::F23),
        // sapp::Keycode::F24          => Some(egui::Key::F24),
        // sapp::Keycode::F25          => Some(egui::Key::F25),
        // sapp::Keycode::Apostrophe   => Some(egui::Key::Apostrophe),
        // sapp::Keycode::Invalid      => Some(egui::Key::Invalid),
        // sapp::Keycode::Comma        => Some(egui::Key::Comma),
        // sapp::Keycode::Period       => Some(egui::Key::Period),
        // sapp::Keycode::Slash        => Some(egui::Key::Slash),
        // sapp::Keycode::Semicolon    => Some(egui::Key::Semicolon),
        // sapp::Keycode::LeftBracket  => Some(egui::Key::LeftBracket),
        // sapp::Keycode::Backslash    => Some(egui::Key::Backslash),
        // sapp::Keycode::RightBracket => Some(egui::Key::RightBracket),
        // sapp::Keycode::GraveAccent  => Some(egui::Key::GraveAccent),
        // sapp::Keycode::World1       => Some(egui::Key::World1),
        // sapp::Keycode::World2       => Some(egui::Key::World2),
        // sapp::Keycode::CapsLock     => Some(egui::Key::CapsLock),
        // sapp::Keycode::ScrollLock   => Some(egui::Key::ScrollLock),
        // sapp::Keycode::NumLock      => Some(egui::Key::NumLock),
        // sapp::Keycode::PrintScreen  => Some(egui::Key::PrintScreen),
        // sapp::Keycode::Pause        => Some(egui::Key::Pause),
        // sapp::Keycode::Menu         => Some(egui::Key::Menu),
        // sapp::Keycode::LeftAlt      => Some(egui::Key::LeftAlt),
        // sapp::Keycode::KpDivide     => Some(egui::Key::KpDivide),
        // sapp::Keycode::RightAlt     => Some(egui::Key::RightAlt),
        // sapp::Keycode::KpDecimal    => Some(egui::Key::KpDecimal),
        // sapp::Keycode::LeftShift    => Some(egui::Key::LeftShift),
        // sapp::Keycode::LeftSuper    => Some(egui::Key::LeftSuper),
        // sapp::Keycode::KpMultiply   => Some(egui::Key::Multiply),
        // sapp::Keycode::RightSuper   => Some(egui::Key::RightSuper),
        // sapp::Keycode::RightShift   => Some(egui::Key::RightShift),
        // sapp::Keycode::LeftControl  => Some(egui::Key::LeftControl),
        // sapp::Keycode::RightControl => Some(egui::Key::RightControl),

        _ => None,
    }
}

pub extern "C" fn event(event: *const sapp::Event) -> bool {
    let state = unsafe { &mut *STATE.as_mut_ptr() };
    let event = unsafe { *event };

    let mut was_handled = false;

    if state.context.wants_keyboard_input() {
        was_handled |= match event._type {
            sapp::EventType::KeyDown => {
                #[rustfmt::skip]
                let mut was_handled = {
                    /*
                        TODO: Verify translation of event.modifiers
                    */
                    state.raw_input.modifiers.alt     = (event.modifiers | sapp::MODIFIER_ALT   as u32) != 0;
                    state.raw_input.modifiers.ctrl    = (event.modifiers | sapp::MODIFIER_CTRL  as u32) != 0;
                    state.raw_input.modifiers.shift   = (event.modifiers | sapp::MODIFIER_SHIFT as u32) != 0;
                    state.raw_input.modifiers.command = (event.modifiers | sapp::MODIFIER_CTRL  as u32) != 0
                                                     || (event.modifiers | sapp::MODIFIER_SUPER as u32) != 0;
                    true
                };

                if let Some(key) = sapp_key_to_egui_key(event.key_code) {
                    state.raw_input.events.push(egui::Event::Key {
                        key,
                        pressed: false,
                        repeat: event.key_repeat,
                        modifiers: state.raw_input.modifiers.clone(),
                    });
                    was_handled = true;
                }

                was_handled
            }

            sapp::EventType::KeyUp => {
                #[rustfmt::skip]
                let mut was_handled = {
                    /*
                        TODO: Verify translation of event.modifiers
                    */
                    state.raw_input.modifiers.alt     = (event.modifiers | sapp::MODIFIER_ALT   as u32) != 0;
                    state.raw_input.modifiers.ctrl    = (event.modifiers | sapp::MODIFIER_CTRL  as u32) != 0;
                    state.raw_input.modifiers.shift   = (event.modifiers | sapp::MODIFIER_SHIFT as u32) != 0;
                    state.raw_input.modifiers.command = (event.modifiers | sapp::MODIFIER_CTRL  as u32) != 0
                                                     || (event.modifiers | sapp::MODIFIER_SUPER as u32) != 0;

                    true
                };

                if let Some(key) = sapp_key_to_egui_key(event.key_code) {
                    state.raw_input.events.push(egui::Event::Key {
                        key,
                        pressed: false,
                        repeat: event.key_repeat,
                        modifiers: state.raw_input.modifiers.clone(),
                    });
                    was_handled = true;
                }

                was_handled
            }

            sapp::EventType::Char => {
                if let Some(char) = char::from_u32(event.char_code) {
                    state
                        .raw_input
                        .events
                        .push(egui::Event::Text(char.to_string()));
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }

    if true | state.context.wants_pointer_input() {
        /*
            TODO: Is this valid? Is there an equivalent of TouchDeviceId in sapp?
        */
        const FAKE_TOUCH_DEVICE_ID: egui::TouchDeviceId = egui::TouchDeviceId(1337);

        was_handled |= match event._type {
            sapp::EventType::MouseDown => {
                state.raw_input.events.push(egui::Event::PointerButton {
                    pos: egui::pos2(event.mouse_x, event.mouse_y),
                    button: sapp_mouse_button_to_egui_pointer_button(event.mouse_button),
                    pressed: false,
                    modifiers: state.raw_input.modifiers.clone(),
                });
                true
            }
            sapp::EventType::MouseUp => {
                state.raw_input.events.push(egui::Event::PointerButton {
                    pos: egui::pos2(event.mouse_x, event.mouse_y),
                    button: sapp_mouse_button_to_egui_pointer_button(event.mouse_button),
                    pressed: true,
                    modifiers: state.raw_input.modifiers.clone(),
                });
                true
            }
            sapp::EventType::MouseScroll => {
                state.raw_input.events.push(egui::Event::Scroll(egui::vec2(
                    event.scroll_x,
                    event.scroll_y,
                )));
                true
            }

            sapp::EventType::MouseMove => {
                state
                    .raw_input
                    .events
                    .push(egui::Event::PointerMoved(egui::pos2(
                        event.mouse_x,
                        event.mouse_y,
                    )));
                true
            }

            sapp::EventType::MouseEnter => {
                state
                    .raw_input
                    .events
                    .push(egui::Event::PointerMoved(egui::pos2(
                        event.mouse_x,
                        event.mouse_y,
                    )));
                true
            }
            sapp::EventType::MouseLeave => {
                state.raw_input.events.push(egui::Event::PointerGone);
                true
            }

            sapp::EventType::TouchesBegan => {
                let mut handled = false;
                for touch in &event.touches[..event.num_touches as usize] {
                    if touch.changed {
                        state.raw_input.events.push(egui::Event::Touch {
                            device_id: FAKE_TOUCH_DEVICE_ID,
                            id: egui::TouchId(touch.identifier as _),
                            phase: egui::TouchPhase::Start,
                            pos: egui::pos2(touch.pos_x, touch.pos_y),
                            force: 0.0,
                        });
                        handled = true;
                    }
                }

                handled
            }
            sapp::EventType::TouchesMoved => {
                let mut handled = false;
                for touch in &event.touches[..event.num_touches as usize] {
                    if touch.changed {
                        state.raw_input.events.push(egui::Event::Touch {
                            device_id: FAKE_TOUCH_DEVICE_ID,
                            id: egui::TouchId(touch.identifier as _),
                            phase: egui::TouchPhase::Move,
                            pos: egui::pos2(touch.pos_x, touch.pos_y),
                            force: 0.0,
                        });
                        handled = true;
                    }
                }

                handled
            }
            sapp::EventType::TouchesEnded => {
                let mut handled = false;
                for touch in &event.touches[..event.num_touches as usize] {
                    if touch.changed {
                        state.raw_input.events.push(egui::Event::Touch {
                            device_id: FAKE_TOUCH_DEVICE_ID,
                            id: egui::TouchId(touch.identifier as _),
                            phase: egui::TouchPhase::End,
                            pos: egui::pos2(touch.pos_x, touch.pos_y),
                            force: 0.0,
                        });
                        handled = true;
                    }
                }

                handled
            }
            sapp::EventType::TouchesCancelled => {
                let mut handled = false;
                for touch in &event.touches[..event.num_touches as usize] {
                    if touch.changed {
                        state.raw_input.events.push(egui::Event::Touch {
                            device_id: FAKE_TOUCH_DEVICE_ID,
                            id: egui::TouchId(touch.identifier as _),
                            phase: egui::TouchPhase::Cancel,
                            pos: egui::pos2(touch.pos_x, touch.pos_y),
                            force: 0.0,
                        });
                        handled = true;
                    }
                }

                handled
            }

            _ => false,
        }
    }

    was_handled |= match event._type {
        sapp::EventType::Resized => {
            /*
                NOTE: In frame(), we fetch the current size, so we will pretend that we are handling
                      since the effect is the same
            */
            true
        }

        sapp::EventType::Iconified | sapp::EventType::Unfocused | sapp::EventType::Suspended => {
            state.raw_input.has_focus = false;
            true
        }
        sapp::EventType::Restored | sapp::EventType::Focused | sapp::EventType::Resumed => {
            state.raw_input.has_focus = true;
            true
        }

        sapp::EventType::QuitRequested => false,
        sapp::EventType::ClipboardPasted => false,
        sapp::EventType::FilesDropped => false,

        sapp::EventType::Num => false,
        sapp::EventType::Invalid => false,

        _ => false,
    };

    was_handled
}
