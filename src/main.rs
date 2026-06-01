mod app;
mod application;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::{Arc, Mutex};

use log::LevelFilter;
use log4rs::{
    append::rolling_file::{
        policy::compound::{
            roll::fixed_window::FixedWindowRoller,
            trigger::size::SizeTrigger,
            CompoundPolicy,
        },
        RollingFileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
};
use postgresql_embedded::PostgreSQL;

fn init_logger() {
    let log_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("aries")
        .join("logs");

    std::fs::create_dir_all(&log_dir).expect("failed to create log directory");

    let log_file    = log_dir.join("aries.log");
    let archive_pat = log_dir.join("aries.{}.log").to_string_lossy().into_owned();

    let roller   = FixedWindowRoller::builder().build(&archive_pat, 5).unwrap();
    let trigger  = SizeTrigger::new(5 * 1024 * 1024); // 5 MB
    let policy   = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let appender = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] {m}\n")))
        .build(log_file, Box::new(policy))
        .expect("failed to build log appender");

    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .expect("failed to build log config");

    log4rs::init_config(config).expect("failed to init logger");
}

// NOTE: only dev; prod must use a proper migration tool
fn run_migrations(client: &mut postgres::Client) {
    let mut entries: Vec<_> = std::fs::read_dir("database/migrations")
        .expect("database/migrations not found")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "sql"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let sql  = std::fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("failed to read {:?}", path));
        client
            .batch_execute(&sql)
            .unwrap_or_else(|e| panic!("failed to run {:?}: {e}", path));
    }
}

fn main() {
    init_logger();
    log::info!("starting aries");

    let rt = tokio::runtime::Runtime::new().unwrap();

    let (pg, url) = rt.block_on(async {
        let mut pg = PostgreSQL::default();
        pg.setup().await.expect("failed to setup postgres");
        pg.start().await.expect("failed to start postgres");

        if !pg.database_exists("aries").await.unwrap_or(false) {
            pg.create_database("aries").await.expect("failed to create database");
        }

        let url = pg.settings().url("aries");
        (pg, url)
    });

    let mut client = postgres::Client::connect(&url, postgres::NoTls)
        .expect("failed to connect to postgres");

    run_migrations(&mut client);

    let client = Arc::new(Mutex::new(client));

    eframe::run_native(
        "Aries",
        eframe::NativeOptions::default(),
        Box::new(move |_cc| Ok(Box::new(app::App::new(client)))),
    )
    .expect("failed to start app");

    log::info!("shutting down aries");

    rt.block_on(async {
        pg.stop().await.expect("failed to stop postgres");
    });
}
