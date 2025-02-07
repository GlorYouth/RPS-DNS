#[cfg_attr(debug_assertions, allow(dead_code))]
#[cfg(debug_assertions)]
pub mod debug {
    use ahash::RandomState;
    use chrono::Local;
    use log::{Level, Record};
    use std::cell::RefCell;
    use std::thread;

    thread_local! {
        static THREAD_LOGGER: RefCell<ThreadLogger> = RefCell::new(ThreadLogger::new());
    }

    struct LogEntry {
        level: Level,
        message: String,
        timestamp: chrono::DateTime<Local>,
        thread_id: u64,
    }

    struct ThreadLogger {
        logs: Vec<LogEntry>,
        thread_id: u64,
        is_println_enabled: bool,
    }

    impl ThreadLogger {
        fn new() -> Self {
            let hash_builder = RandomState::with_seed(42);
            let hash = hash_builder.hash_one(thread::current().id());
            Self {
                logs: Vec::new(),
                thread_id: hash,
                is_println_enabled: false,
            }
        }

        fn log(&mut self, record: &Record) {
            let entry = LogEntry {
                level: record.level(),
                message: record.args().to_string(),
                timestamp: Local::now(),
                thread_id: self.thread_id,
            };
            self.logs.push(entry);
        }

        fn set_println_enabled(&mut self, value: bool) {
            self.is_println_enabled = value;
        }
    }

    pub struct GlobalLogger;

    impl GlobalLogger {
        pub fn new() -> GlobalLogger {
            // 设置 env_logger 的格式包含线程ID
            GlobalLogger {}
        }
    }

    impl log::Log for GlobalLogger {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            // 控制日志级别，可根据需要调整
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &Record) {
            // 将日志记录到当前线程的本地存储
            THREAD_LOGGER.with(|logger| {
                logger.borrow_mut().log(record);
                // 使用 env_logger 打印日志（确保已初始化）;
                if logger.borrow().is_println_enabled {
                    println!(
                        "{}",
                        format!(
                            "[{}] [{} {:X}]  {}",
                            record.level(),
                            Local::now().format("%Y-%m-%d %H:%M:%S"),
                            logger.borrow().thread_id,
                            record.args()
                        )
                    );
                }
            });
        }

        fn flush(&self) {
            THREAD_LOGGER.with(|logger| {
                logger.borrow_mut().logs.clear();
            });
        }
    }

    pub fn init_logger() {
        // 设置全局 logger
        if log::set_logger(Box::leak(Box::from(GlobalLogger::new()))).is_ok() {
            std::panic::set_hook(Box::new(|info| {
                eprintln!("Test panicked: {:?}", info);

                // 获取当前线程的日志并打印
                let logs = get_current_thread_logs();
                if !logs.is_empty() {
                    eprintln!("Captured logs before panic:");
                    for log in logs {
                        eprintln!("{}", log);
                    }
                }
            }));
        } else {
            logger_flush();
        }
        log::set_max_level(log::LevelFilter::Trace);
    }

    pub fn logger_flush() {
        log::logger().flush();
    }

    pub fn get_current_thread_logs() -> Vec<String> {
        THREAD_LOGGER.with(|logger| {
            logger
                .borrow()
                .logs
                .iter()
                .map(|entry| {
                    format!(
                        "[{}] {}",
                        entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        entry.message
                    )
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod test {
    use super::debug::*;
    use log::debug;

    #[test]
    fn test() {
        init_logger();
        debug!("hello trace");
        println!("{:?}", get_current_thread_logs());
        logger_flush()
    }
}
