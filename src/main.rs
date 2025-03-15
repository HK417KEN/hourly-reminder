
// 隐藏编译后打开程序的 CMD 窗口
#![cfg_attr(
    all(target_os = "windows", not(debug_assertions)),
    windows_subsystem = "windows"
)]


use std::{env, fs, io::BufReader, path};

mod auto_launch;
mod tray_icon_tao;
mod timer;



static ASSETS_DIR_NAME: &str = "assets";



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    #[cfg(not(target_os = "windows"))]
    panic!("Not Implemented yet!");

    println!("running");

    init()
}

// 初始化程序
fn init() -> Result<(), Box<dyn std::error::Error>> {

    // CD 到应用目录
    cd_to_app_path()?;

    // 程序运行时马上播放一次音效
    play_audio()?;

    // 托盘图标和定时功能
    tray_icon_tao::run()?;

    Ok(())
}

// CD 到应用目录
pub fn cd_to_app_path() -> Result<(), std::io::Error> {
    let exe_path = env::current_exe().unwrap();
    env::set_current_dir(
        exe_path.parent()
            .map(|p| p.to_path_buf()) // 如果有父目录，转换为 PathBuf
            .unwrap_or(exe_path) // 如果没有父目录，直接使用原始路径
    )
}

pub fn get_assets_path() -> path::PathBuf {
    path::Path::new(ASSETS_DIR_NAME).to_path_buf()
}

// 播放音效
pub fn play_audio() -> Result<(), Box<dyn std::error::Error>> {

    tokio::spawn( async {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&stream_handle).unwrap();
        let path: path::PathBuf;

        if fs::exists("audio.ogg").is_ok_and(|bool| bool) {
            path = "audio.ogg".into();
        } else if fs::exists("audio.wav").is_ok_and(|bool| bool) {
            path = "audio.wav".into();
        } else if fs::exists(get_assets_path().join("audio.ogg")).is_ok_and(|bool| bool) {
            path = get_assets_path().join("audio.ogg");
        } else if fs::exists(get_assets_path().join("audio.wav")).is_ok_and(|bool| bool) {
            path = get_assets_path().join("audio.wav");
        } else if fs::exists("default.ogg").is_ok_and(|bool| bool) {
            path = "default.ogg".into();
        } else if fs::exists(get_assets_path().join("default.ogg")).is_ok_and(|bool| bool) {
            path = get_assets_path().join("default.ogg");
        } else {
            panic!("未找到音频文件!");
        }

        let file = fs::File::open(path).unwrap();
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

        sink.sleep_until_end();
    });

    Ok(())
}
