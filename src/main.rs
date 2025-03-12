
// 隐藏编译后打开程序的CMD窗口
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]


use auto_launch::AutoLaunch;
use std::{env::current_exe, io::BufReader};

mod tray_icon_tao;

mod timer;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[cfg(not(target_os = "windows"))]
    panic!("Not Implemented yet!");

    println!("running");

    init()
}

// 初始化程序
pub fn init() -> Result<(), Box<dyn std::error::Error>> {

    // 程序运行时马上播放一次音效
    play_audio()?;

    // 托盘图标和定时功能
    tray_icon_tao::run()?;

    Ok(())
}

// 检查是否已启用 开机自启
pub fn check_is_enabled_auto_launch() -> bool {
    let current_exe = current_exe().unwrap();
    let app_name = env!("CARGO_PKG_NAME");
    let app_path = &current_exe.display().to_string();
    let args = &[""];
    let auto = AutoLaunch::new(app_name, app_path, args);

    auto.is_enabled().unwrap()
}

// 开机自启
pub fn auto_launch(auto_launch: bool) -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = current_exe()?;
    let app_name = env!("CARGO_PKG_NAME");
    let app_path = &current_exe.display().to_string();
    let args = &[""];
    let auto = AutoLaunch::new(app_name, app_path, args);

    let check_is_enabled_auto_launch = check_is_enabled_auto_launch();

    if auto_launch && !check_is_enabled_auto_launch {
        // 启用 开机自启
        auto.enable()?;
    } else if !auto_launch && check_is_enabled_auto_launch {
        // 禁用 开机自启
        auto.disable()?;
    }

    Ok(())
}

// 播放音效
pub fn play_audio() -> Result<(), Box<dyn std::error::Error>> {

    tokio::spawn( async {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let path: &'static str;
    
        if std::fs::exists("audio.ogg").is_ok_and(|bool| bool) {
            path = "audio.ogg";
        } else if std::fs::exists("audio.wav").is_ok_and(|bool| bool) {
            path = "audio.wav";
        } else if std::fs::exists("assets/audio.ogg").is_ok_and(|bool| bool) {
            path = "assets/audio.ogg";
        } else if std::fs::exists("assets/audio.wav").is_ok_and(|bool| bool) {
            path = "assets/audio.wav";
        } else if std::fs::exists("default.ogg").is_ok_and(|bool| bool) {
            path = "default.ogg";
        } else if std::fs::exists("assets/default.ogg").is_ok_and(|bool| bool) {
            path = "assets/default.ogg";
        } else {
            panic!("未找到音频文件!");
        }
    
        let file = std::fs::File::open(path).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    
        sink.sleep_until_end();
    });

    Ok(())
}
