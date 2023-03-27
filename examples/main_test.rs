use sokol::app as sapp;
use sokol::gfx as sg;
use sokol::glue as sglue;
use sokol::log as slog;

use sokol_rust_egui as segui;

struct State {
    bool_value: bool,
}

extern "C" fn egui_frame(ctx: *const egui::Context, userdata: *mut core::ffi::c_void) {
    let state = unsafe { &mut *(userdata as *mut State) };
    let ctx = unsafe { &*ctx };

    egui::Window::new("Hello, World!").show(ctx, |ui| {
        ui.label("this is a label!");
        if ui.button("button!").clicked() {
            println!("Clicked button!");
            state.bool_value = !state.bool_value;
        }

        ui.add_space(10.0);
        ui.heading("A Heading!");
        ui.horizontal(|ui| {
            ui.checkbox(&mut state.bool_value, "check it out 1");
            ui.checkbox(&mut state.bool_value, "check it out 2");
            ui.checkbox(&mut state.bool_value, "check it out 3");
            ui.checkbox(&mut state.bool_value, "check it out 4");
        });
        ui.vertical(|ui| {
            let alt = !state.bool_value;
            ui.radio_value(&mut state.bool_value, alt, "radio 1");
            ui.radio_value(&mut state.bool_value, alt, "radio 2");
            ui.radio_value(&mut state.bool_value, alt, "radio 3");
            ui.radio_value(&mut state.bool_value, alt, "radio 4");
        });
        ui.add_space(10.0);
    });

    /*
        TODO: Include egui demo library here
    */
}

extern "C" fn frame(userdata: *mut core::ffi::c_void) {
    let state = unsafe { &mut *(userdata as *mut State) };

    segui::frame();

    let mut pass_action = sg::PassAction::default();
    pass_action.colors[0] = sg::ColorAttachmentAction {
        action: sg::Action::Clear,
        value: sg::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    };

    sg::begin_default_pass(&pass_action, sapp::width(), sapp::height());
    segui::draw();
    sg::end_pass();
    sg::commit();
}

extern "C" fn event(event: *const sapp::Event) {
    let segui_handled_event = segui::event(event);

    if !segui_handled_event {
        let event = unsafe { &*event };
        println!("segui didn't handle: {:?}", event._type)
    }
}

extern "C" fn init(userdata: *mut core::ffi::c_void) {
    sg::setup(&sg::Desc {
        context: sglue::context(),
        logger: sg::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });

    segui::begin(&segui::Desc {
        gui_userdata_cb: Some(egui_frame),
        userdata,
        ..Default::default()
    });
}

extern "C" fn cleanup() {
    segui::shutdown();
    sg::shutdown();
}

fn main() {
    let mut state = State { bool_value: false };

    sapp::run(&sapp::Desc {
        frame_userdata_cb: Some(frame),
        init_userdata_cb: Some(init),
        cleanup_cb: Some(cleanup),
        event_cb: Some(event),

        user_data: &mut state as *mut _ as *mut _,

        width: 800,
        height: 600,

        window_title: b"sokol-rust-egui example\0".as_ptr() as _,

        logger: sapp::Logger {
            func: Some(slog::slog_func),
            ..Default::default()
        },

        ..Default::default()
    });
}
