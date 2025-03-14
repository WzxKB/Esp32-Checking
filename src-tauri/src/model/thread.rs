use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// 全局线程管理器
lazy_static! {
    static ref THREAD_MANAGER: Arc<Mutex<ThreadManager>> = Arc::new(Mutex::new(ThreadManager {
        threads: HashMap::new(),
    }));
}

// 线程管理器结构
struct ThreadManager {
    threads: HashMap<String, ThreadInfo>,
}

// 线程信息结构
struct ThreadInfo {
    handle: JoinHandle<()>,
    stop_signal: Arc<AtomicBool>,
}

/// 启动线程
/// thread_id: 线程唯一标识
/// callback: 线程执行的回调函数
/// sleep_ms: 每次循环的休眠时间（毫秒）
pub fn start_thread<F>(thread_id: &str, callback: F, sleep_ms: u64) -> Result<(), String>
where
    F: Fn() + Send + 'static,
{
    let mut manager = THREAD_MANAGER.lock().map_err(|e| e.to_string())?;

    // 检查线程是否已存在
    if manager.threads.contains_key(thread_id) {
        return Err(format!("Thread '{}' already exists", thread_id));
    }

    // 创建停止信号
    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_clone = stop_signal.clone();
    let id = thread_id.to_string().clone();
    // 创建线程
    let handle = thread::spawn(move || {
        loop {
            // 检查停止信号
            if stop_signal_clone.load(Ordering::Relaxed) {
                println!("[{}] Thread stopping...", id);
                break;
            }

            // 执行工作回调
            callback();

            // 休眠
            thread::sleep(Duration::from_millis(sleep_ms));
        }
    });

    // 保存线程信息
    manager.threads.insert(
        thread_id.to_string(),
        ThreadInfo {
            handle,
            stop_signal,
        },
    );

    println!("[{}] Thread started", thread_id);
    Ok(())
}

/// 停止指定线程
pub fn stop_thread(thread_id: &str) -> Result<(), String> {
    // 从管理器移除线程信息
    let thread_info = {
        let mut manager = THREAD_MANAGER.lock().map_err(|e| e.to_string())?;
        manager.threads.remove(thread_id)
    };

    if let Some(info) = thread_info {
        // 设置停止信号
        info.stop_signal.store(true, Ordering::Relaxed);

        // 等待线程结束
        info.handle
            .join()
            .map_err(|e| format!("Failed to join thread: {:?}", e))?;

        println!("[{}] Thread stopped", thread_id);
        Ok(())
    } else {
        Err(format!("Thread '{}' not found", thread_id))
    }
}

/// 检查线程是否存在
#[allow(unused)]
pub fn thread_exists(thread_id: &str) -> bool {
    let manager = THREAD_MANAGER.lock().unwrap();
    manager.threads.contains_key(thread_id)
}
