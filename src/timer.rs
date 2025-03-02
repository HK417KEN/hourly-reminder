use chrono::{DateTime, Local, Timelike};
use tokio::sync::watch;
use tokio::time::{self, Duration, Instant, MissedTickBehavior};

/*
#[tokio::main]
async fn main() {
    let timer_switch = TimerSwitch::new();
    let handle = timer_switch.spawn_task();

    // 立即启用
    println!("[CTRL] 启用定时任务");
    timer_switch.enable();

    // 运行2小时（演示用）
    tokio::time::sleep(Duration::from_secs(7200)).await;
    
    // 禁用任务
    println!("[CTRL] 禁用定时任务");
    timer_switch.disable();
    
    handle.await.unwrap();
}
*/

pub struct TimerSwitch {
    sender: watch::Sender<bool>,
}

impl TimerSwitch {

    // 新建实例（创建 观察者）
    pub fn new() -> Self {
        let (sender, _) = watch::channel(false);
        Self { sender }
    }

    // 发送 启用 信号
    pub fn enable(&self) {
        let _ = self.sender.send(true);
    }

    // 发送 停用 信号
    pub fn disable(&self) {
        let _ = self.sender.send(false);
    }

    // 执行任务
    pub fn spawn_task(&self) -> tokio::task::JoinHandle<()> {
        let mut receiver = self.sender.subscribe();
        tokio::spawn(async move {
            loop {
                if *receiver.borrow_and_update() {
                    // 接收到 启用 信号

                    loop {
                        // 计算第一个触发时间
                        let start = next_start_of_hour(Local::now());
                        let mut interval = time::interval_at(
                            Instant::now() + (start - Local::now()).to_std().unwrap(),
                            Duration::from_secs(60)
                        );
                        
                        // 关键配置：跳过积压的触发
                        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

                        tokio::select! {
                            _ = interval.tick() => {
                                println!("Exec time: {}", Local::now().format("%H:%M:%S"));
                                crate::play_audio().unwrap();
                            }
                            changed = receiver.changed() => {
                                if changed.is_err() || !*receiver.borrow() {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    // 接收到 停用 信号

                    // 等待状态变化
                    if receiver.changed().await.is_err() {
                        return;
                    }
                }
            }
        })
    }
}

// 计算下一个整分钟的起始时刻
fn next_start_of_hour(now: DateTime<Local>) -> DateTime<Local> {
    let base = now.with_second(0).unwrap().with_nanosecond(0).unwrap();
    if base <= now {
        base + tokio::time::Duration::from_secs(60 * 60)
    } else {
        base
    }
}
