// Copyright 2022-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//
// Modified by [KEN]



use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoopBuilder},
};
use tray_icon::{
    menu::{AboutMetadata, CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder /* , TrayIconEvent, */
};

enum UserEvent {
    //TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();

    // set a tray event handler that forwards the event and wakes up the event loop
    /*
    let proxy = event_loop.create_proxy();
    TrayIconEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::TrayIconEvent(event));
    }));
    */

    // set a menu event handler that forwards the event and wakes up the event loop
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        let _ = proxy.send_event(UserEvent::MenuEvent(event));
    }));

    let tray_menu = Menu::new();

    let set_enabled_ci = CheckMenuItem::new("已启用", true, true, None);
    let set_auto_launch_ci = CheckMenuItem::new("开机自启", true, true, None);
    let about_i = PredefinedMenuItem::about(
        Some("关于"),
        Some(AboutMetadata {
            name: Some("hourly reminder - [KEN]".to_string()),
            ..Default::default()
        }),
    );
    let quit_i = MenuItem::new("退出", true, None);

    let _ = tray_menu.append_items(&[
        &set_enabled_ci,
        &PredefinedMenuItem::separator(),
        &set_auto_launch_ci,
        &PredefinedMenuItem::separator(),
        &about_i,
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    let mut tray_icon = None;

    let _ = MenuEvent::receiver();
    // let _ = TrayIconEvent::receiver();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(tao::event::StartCause::Init) => {
                let icon = load_icon(std::path::Path::new(path));

                // We create the icon once the event loop is actually running
                // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
                tray_icon = Some(
                    TrayIconBuilder::new()
                        .with_menu(Box::new(tray_menu.clone()))
                        .with_tooltip("hourly reminder")
                        .with_icon(icon)
                        .build()
                        .unwrap(),
                );

                // We have to request a redraw here to have the icon actually show up.
                // Tao only exposes a redraw method on the Window so we use core-foundation directly.
                #[cfg(target_os = "macos")]
                unsafe {
                    use objc2_core_foundation::{CFRunLoopGetMain, CFRunLoopWakeUp};

                    let rl = CFRunLoopGetMain().unwrap();
                    CFRunLoopWakeUp(&rl);
                }
            }

            /*
            Event::UserEvent(UserEvent::TrayIconEvent(event)) => {
                println!("{event:?}");
            }
            */

            Event::UserEvent(UserEvent::MenuEvent(event)) => {
                println!("{event:?}");

                if event.id == set_enabled_ci.id() {

                    if set_enabled_ci.is_checked() {
                        // continue reminder
                        println!("continue reminder");
                        set_enabled_ci.set_text("已启用");
                    } else {
                        // stop reminder
                        println!("stop reminder");
                        set_enabled_ci.set_text("未启用");
                    }

                } else if event.id == set_auto_launch_ci.id() {

                    if set_auto_launch_ci.is_checked() {
                        // set auto launch
                        println!("set auto launch");
                    } else {
                        // unset auto launch
                        println!("unset auto launch");
                    }

                } else if event.id == quit_i.id() {
                    tray_icon.take();
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }
    })
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}