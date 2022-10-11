use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

const LOG_PATH: &str = "logs/current.log";
const ROLLING_PATTERN: &str = "logs/rusty_{}.log";
// 5 mb
const MAX_ROLLING_FILE_SIZE: u64 = 5 * 1024 * 1024;
const MAX_ROLL_COUNT: u32 = 5;

pub fn setup_logger() {
    log4rs::init_file("config/log.yaml", Default::default()).unwrap_or_else(|_| {
        let trigger = Box::new(SizeTrigger::new(MAX_ROLLING_FILE_SIZE));

        let roller = Box::new(
            FixedWindowRoller::builder()
                .base(1)
                .build(ROLLING_PATTERN, MAX_ROLL_COUNT)
                .unwrap(),
        );

        let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

        let file = RollingFileAppender::builder()
            .encoder(Box::new(PatternEncoder::default()))
            .build(LOG_PATH, compound_policy)
            .unwrap();

        let stdout = ConsoleAppender::builder().build();

        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .logger(
                Logger::builder()
                    .build("rusty_controller", LevelFilter::Debug),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("file")
                    .build(LevelFilter::Warn),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();

        log_panics::init()
    });
}
