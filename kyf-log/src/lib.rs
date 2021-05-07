#![allow(unused_imports)]
use chrono::Local;
use std::io::Write;

pub struct Logger<F> {
    hooks: Vec<F>,
}

impl<F> Logger<F>
where
    F: Fn(log::Level, &String) + Sync + Send + 'static,
{
    pub fn new() -> Self {
        let h: Vec<F> = Vec::new();
        Logger { hooks: h }
    }

    pub fn init(self) {
        env_logger::Builder::from_default_env()
            .format(move |buf, record| {
                let body = format!(
                    r#"{{"ts": "{}", "level": "{}", "file": "{}", "line": {}, {}}}"#,
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.file().unwrap_or(""),
                    record.line().unwrap_or(0),
                    record.args(),
                );
                self.hooks.iter().for_each(|h| {
                    h(record.level(), &body);
                });
                writeln!(buf, "{}", body)
            })
            .init();
    }

    pub fn add_hook(&mut self, f: F) {
        self.hooks.push(f);
    }
}

#[macro_export]
macro_rules! myinfo {
    ($l:expr, $tag:expr; $args:tt) => {
        info!(
            r#""msg": "{}", "data": {}"#,
            format!($l, $tag),
            json!($args)
        );
    };
    ($l:expr, $tag:expr) => {
        info!(r#""msg: "{}""#, format!($l, $tag));
    };
    ($l:expr) => {
        info!(r#""msg": "{}""#, $l);
    };
    ($l:expr; $args:tt) => {
        info!(r#""msg": "{}", "data": {}"#, $l, json!($args));
    };
}

#[macro_export]
macro_rules! myerror {
    ($l:expr, $tag:expr; $args:tt) => {
        error!(r#""msg": "{}", "data": {}"#, $l, json!($args));
    };
    ($l:expr, $tag:expr) => {
        error!(r#""msg: "{}""#, format!($l, $tag));
    };
    ($l:expr) => {
        error!(r#""msg": "{}""#, $l);
    };
    ($l:expr; $args:tt) => {
        error!(r#""msg": "{}", "data": {}"#, $l, json!($args));
    };
}

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_json;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        std::env::set_var("RUST_LOG", "INFO");
        let mut logger = super::Logger::new();
        logger.add_hook(|lvl, body| match lvl {
            log::Level::Error => {
                println!("这个是个错误：{}", body);
            }
            _ => {}
        });
        logger.init();

        myinfo!("这个数据是{}？？？", "haha"; {"name": "123455"});
        myerror!("这个gas的"; {"age": 10});
    }
}
