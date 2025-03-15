
use std::env;

use auto_launch::AutoLaunch;

// 获取新的自启动实例
pub fn new() -> Result<AutoLaunch, Box<dyn std::error::Error + 'static>> {
    let current_exe = env::current_exe()?;
    let app_name = env!("CARGO_PKG_NAME");
    let app_path = &current_exe.display().to_string();
    let args = &[""];
    Ok(AutoLaunch::new(app_name, app_path, args))
}

// 设置是否开机自启
pub fn set(is_set_to_enable: bool) -> Result<(), Box<dyn std::error::Error>> {
    let auto_launch = new()?;

    let is_enabled = auto_launch.is_enabled()?;

    if is_set_to_enable && !is_enabled {
        // 启用 开机自启
        auto_launch.enable()?;
    } else if !is_set_to_enable && is_enabled {
        // 禁用 开机自启
        auto_launch.disable()?;
    }

    Ok(())
}
