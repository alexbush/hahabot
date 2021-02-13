use carapax::{
    Api,
    Config,
    Dispatcher,
    longpoll::LongPoll,
};
use dotenv::dotenv;
use env_logger;
use tokio::sync::Mutex;
use std::{env, sync::Arc};

use hahabot::commands::*;
use hahabot::Context;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let token  = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let config = Config::new(token);
    let api    = Api::new(config).expect("Failed to create API");
    let count  = Arc::new(Mutex::new(0));

    let mut dispatcher = Dispatcher::new(Context {
        api: api.clone(),
        count: count.clone(),
    });

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
