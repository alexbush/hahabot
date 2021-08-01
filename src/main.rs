use carapax::{
    access::{
        AccessHandler,
        AccessRule,
        InMemoryAccessPolicy,
        PrincipalChat
    },
    Api,
    Config,
    Dispatcher,
    longpoll::LongPoll,
    LoggingErrorHandler,
    ErrorPolicy,
    ErrorHandler,
    HandlerError,
};
use async_trait::async_trait;
use env_logger;
use log::info;
use tokio::sync::Mutex;
use std::{
    sync::Arc,
    fs::read_to_string
};
use serde_derive::Deserialize;

use hahabot::commands::*;
use hahabot::Context;
use hahabot::DtpCache;

#[derive(Debug, Deserialize)]
struct BotConfig {
    token: String,
    chats: Option<Vec<i64>>,
    users: Option<Vec<String>>,
}

struct MyErrorHandler;

#[async_trait]
impl ErrorHandler for MyErrorHandler {
    async fn handle(&mut self, _: HandlerError) -> ErrorPolicy {
        ErrorPolicy::Continue
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let config_string  = read_to_string("config.toml")?;
    let cfg: BotConfig = toml::from_str(&config_string)?;
    info!("{:?}", cfg);
    let config         = Config::new(cfg.token);
    let api            = Api::new(config).expect("Failed to create API");
    let count          = Arc::new(Mutex::new(0));


    let mut dispatcher = Dispatcher::new(Context {
        api:             api.clone(),
        count:           count.clone(),
        dtp_cache:       Arc::new(Mutex::new(DtpCache {
            last_update: 0,
            header:      String::new(),
            body:        String::new(),
        }))
    });

    dispatcher.set_error_handler(LoggingErrorHandler::new(ErrorPolicy::Continue));

    if cfg.chats.is_some() {
        let mut rules: Vec<AccessRule> = Vec::new();

        for chat in cfg.chats.unwrap().iter() {
            rules.push( AccessRule::allow_chat(PrincipalChat::Id(*chat)) );
        }

        let policy = InMemoryAccessPolicy::new(rules);
        dispatcher.add_handler(AccessHandler::new(policy));
    }


    dispatcher.set_error_handler(MyErrorHandler);

    dispatcher.add_handler(handle_start);
    dispatcher.add_handler(handle_stop);
    dispatcher.add_handler(handle_corona);
    dispatcher.add_handler(handle_anekdot);
    dispatcher.add_handler(handle_bashorg);
    dispatcher.add_handler(handle_ithappens);
    dispatcher.add_handler(handle_mem);
    dispatcher.add_handler(handle_me);
    dispatcher.add_handler(handle_help);
    dispatcher.add_handler(handle_dtp);
    dispatcher.add_handler(handle_count);

    LongPoll::new(api, dispatcher).run().await;

    Ok(())
}
