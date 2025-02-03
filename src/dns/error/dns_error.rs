#[cfg_attr(debug_assertions, allow(dead_code))]
#[cfg(debug_assertions)]
pub mod debug {
    use chrono::Local;
    use log::{Level, Record};
    use std::cell::RefCell;
    use std::thread;
    use ahash::RandomState;
    

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
    }

    impl ThreadLogger {
        fn new() -> Self {
            let hash_builder = RandomState::with_seed(42);
            let hash = hash_builder.hash_one(thread::current().id());
            Self {
                logs: Vec::new(),
                thread_id: hash,
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
                println!("{}", format!(
                    "[{}] [{} {:X}]  {}",
                    record.level(),
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    logger.borrow().thread_id,
                    record.args()
                ));
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
        log::set_boxed_logger(Box::new(GlobalLogger::new())).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    }
    
    pub fn logger_flush() {
        log::logger().flush();
    }

    pub fn get_current_thread_logs() -> Vec<String> {
        THREAD_LOGGER.with(|logger| {
            logger.borrow().logs.iter().map(|entry| {
                format!(
                    "[{}] {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    entry.message
                )
            }).collect()
        })
    }

}

#[cfg(test)]
mod test {
    use log::debug;
    use crate::dns::error::{logger_flush, init_logger, get_current_thread_logs};

    #[test]
    fn test() {
        init_logger();
        debug!("hello trace");
        println!("{:?}", get_current_thread_logs());
        logger_flush()
    }
}
