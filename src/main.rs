use auto_launch::AutoLaunch;
use std::{env::current_exe, io::BufReader, time::Duration};
use tao::event_loop::{EventLoopBuilder, ControlFlow};
use tray_icon::{menu::{Menu, MenuEvent, MenuItem}, Icon, TrayIconBuilder, TrayIconEvent};

mod tray_icon_tao;

// const ICON: &[u8] = include_bytes!("../icon.png");



fn main() {

    #[cfg(not(target_os = "windows"))]
    panic!("Not Implemented yet!");

    println!("Hello, world!");

    match init() {
        Ok(_) => (),
        Err(e) => panic!("{}", e)
    }
}

pub fn init() -> Result<(), Box<dyn std::error::Error>> {

    play_audio()?;

    // init_sys_tray()?;
    tray_icon_tao::main()?;

    // auto_launch(false)?;

    Ok(())
}

pub fn auto_launch(auto_launch: bool) -> Result<(), Box<dyn std::error::Error>> {

    let current_exe = current_exe()?;

    let app_name = env!("CARGO_PKG_NAME");
    let app_path = &current_exe.display().to_string();
    let args = &[""];
    let auto = AutoLaunch::new(app_name, app_path, args);

    if auto_launch {
        // enable the auto launch
        auto.enable()?;
        auto.is_enabled().unwrap();
    } else {
        // disable the auto launch
        auto.disable()?;
        auto.is_enabled().unwrap();
    }

    Ok(())
}

/*
fn load_icon() -> Result<Icon, Box<dyn std::error::Error>> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory_with_format(ICON, image::ImageFormat::Png)?.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)?)
}
*/

pub fn init_sys_tray () -> Result<(), Box<dyn std::error::Error>> {

    let tray_menu = Menu::new();
    tray_menu.append(&MenuItem::new("退出", true, None))?;


    println!("loading icon");

    //let icon = load_icon()?;
    let icon = Icon::from_path("icon.ico", Some((500, 500)))?;

    println!("icon loaded");

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("system-tray - tray icon library!")
        .with_icon(icon)
        .build()?;
    
    let event_loop = EventLoopBuilder::new().build();
    
    let proxy = event_loop.create_proxy();
    std::thread::spawn(move || {
        loop {
            proxy.send_event(()).ok();
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            println!("tray event: {:?}", event);
        }

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            println!("menu event: {:?}", event);
        }

    });

}

fn play_audio() -> Result<(), Box<dyn std::error::Error>> {

    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&stream_handle)?;
    let path: &'static str;

    if std::fs::exists("audio.ogg").is_ok_and(|bool| bool == true) {
        path = "assets/audio.ogg";
    } else if std::fs::exists("audio.wav").is_ok_and(|bool| bool == true) {
        path = "assets/audio.wav";
    } else if std::fs::exists("assets/default.ogg").is_ok_and(|bool| bool == true) {
        path = "assets/default.ogg";
    } else {
        panic!("Audio not found!");
    }

    let file = std::fs::File::open(path)?;
    sink.append(rodio::Decoder::new(BufReader::new(file))?);

    sink.sleep_until_end();

    Ok(())
}
