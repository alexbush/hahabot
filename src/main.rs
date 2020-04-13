use carapax::{
    handler, 
    longpoll::LongPoll, 
    methods::{ SendMessage, DeleteMessage}, 
    Api, 
    types::{Command, ParseMode},
    Config, 
    Dispatcher, 
    ExecuteError, 
    HandlerResult,
};
use dotenv::dotenv;
use env_logger;
use log;
use time;
use std::env;

mod memory;
mod sources;
mod corona;

#[handler(command = "/start")]
async fn handle_start(api: &Api, command: Command) -> Result<HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    match memory::create_table(chat_id) {
        Ok(_) => log::info!("table {} created", chat_id),
        Err(why) => log::warn!("can't create table {}: {}", chat_id, why)
    }
    let method = SendMessage::new(chat_id, "Всем чьмоки в этом чате!");
    api.execute(method).await?;
    Ok(HandlerResult::Stop)
}

#[handler(command = "/me")]
async fn handle_me(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let message = command.get_message();
    let chat_id = message.get_chat_id();

    let user: String = match message.get_user() {
        Some(u) => match u.username.clone() {
            Some(n) => format!("@{}", n),
            None => u.first_name.clone()
        },
        None => "Someone".to_string()
    };

    let user_message: String = match message.get_text() {
        Some(text) => text.data.clone().replace("/me ", ""),
        None => "something".to_string(),
    };

    match api.execute(DeleteMessage::new(chat_id, message.id)).await {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }
    
    api.execute(SendMessage::new(chat_id, format!("{} {}", user, user_message))).await?;

    Ok(())
}

#[handler(command = "/m")]
async fn handle_mem(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let message = command.get_message();
    
    println!("\n\n {:#?}\n\n", message);

    let chat_id = message.get_chat_id();

    let args = command.get_args();

    let answer = if args.is_empty() {
        let reply = message.reply_to.clone();
        log::info!("FORWARD {:?}", reply);
        match reply {
            Some(f) => match f.get_text() {
                Some(text) => { 
                    log::info!("F {:?}", f);
                    log::info!("TEXT {:?}", text);
                    let me = memory::Message {
                        id: 0,
                        message: text.data.clone(),
                        // FIXME: replace Anonym whith real name or id
                        author: "Anonymous".to_string(),
                        created: time::get_time(),
                    };

                    match memory::save(chat_id, &me) {
                        Ok(_) => format!("saved"),
                        Err(err) => {
                            log::warn!("Cant save message: {}", err);
                            "[:||||:]".to_string()
                        }
                    }
                }
                None => "forward message not found".to_string()
            },
            None => {
                log::info!("f.get_text not found");
                match memory::get_random(chat_id) {
                    Ok(msg) => msg.message,
                    Err(err) => {
                        log::warn!("cant get random message: {}", err);
                        "not found".to_string()
                    }
                }
            }
        }
    } else {
        let id: i64 = args[0].parse::<i64>().unwrap_or(0);
        match memory::get(chat_id, id) {
            Ok(m) => m.message,
            Err(err) => {
                log::warn!("cant get {} message: {}", id, err);
                if id == 0 {
                    match message.get_text() {
                        Some(text) => {
                            let me = memory::Message {
                                id: 0,
                                message: text.data.clone().replace("/m ", ""),
                                // FIXME: replace Anonym whith real name or id
                                author: "Anonymous".to_string(),
                                created: time::get_time(),
                            };
                            match memory::save(chat_id, &me) {
                                Ok(_) => format!("saved"),
                                Err(err) => {
                                    log::warn!("Cant save message: {}", err);
                                    "[:||||:]".to_string()
                                }
                            }
                        },
                        None => "".to_string()
                    }
                } else {
                    format!("{} not found", id)
                }
            }
        }
    };

    api.execute(SendMessage::new(chat_id, answer)).await?;
    Ok(())
}

#[handler(command = "/stop")]
async fn handle_stop(api: &Api, command: Command) -> Result<(), ExecuteError> {
    log::info!("handle /stop command\n");
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "Cant stop me now!");
    api.execute(method).await?;
    Ok(())
}

#[handler(command = "/it")]
async fn handle_ithappens(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    
    let answer = match sources::ithappens() {
        Ok(body) => body,
        Err(_) => "not found".to_string(),
    };
    
    api.execute(SendMessage::new(chat_id, answer)).await?;
    Ok(())
}

#[handler(command = "/a")]
async fn handle_anekdot(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let answer = match sources::anekdot() {
        Ok(body) => body,
        Err(_) => "not found".to_string(),
    };

    api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}

#[handler(command = "/corona")]
async fn handle_corona(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();

    let answer: String = if args.is_empty() {
        match corona::latest() {
            Ok(c) => c,
            Err(why) => {
                log::error!("Cat't parse all data from api: {}", why);
                format!("Something went wrong with api")
            }
        }
    } else {
        match args[0].as_str() {
            "top" => if args.len() > 1 {
                    match corona::top(args[1].to_string()) {
                        Ok(c) => (c),
                        Err(why) => {
                            log::error!("Cat't parse top data from api: {}", why);
                            format!("Something went wrong with api")
                        },
                    } 
                } else {
                    match corona::top("cases".to_string()) {
                        Ok(c) => (c),
                        Err(why) => {
                            log::error!("Cat't parse top data from api: {}", why);
                            format!("Something went wrong with api")
                        },
                    }
                },
            _ => match corona::latest_country(&args[0].to_string()) {
                Ok(c) => c,
                Err(why) => {
                    log::error!("Cat't parse all data from api: {}", why);
                    format!("Something went wrong with api or can't find this country")
                }
            }
        }
    };
    
    api.execute(SendMessage::new(chat_id, answer)
        .parse_mode(ParseMode::MarkdownV2)
    ).await?;

    Ok(())
}

#[handler(command = "/b")]
async fn handle_bashorg(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();

    let answer = if args.is_empty() {
        match sources::bash(0) {
            Ok(body) => body,
            Err(_) => "not found".to_string(),
        }
    } else {
        match sources::bash(args[0].parse::<u64>().unwrap_or(0)) {
            Ok(body) => body,
            Err(_) => "not found".to_string(),
        }
    };

    api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}


#[handler(command = "/h")]
async fn handle_help(api: &Api, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let help_msg = r"
/h - вызов справки
/m on reply - сохранить на память
/m - рандомно вызвать запись из памяти
/m num - запись из памяти
/m str - сохранить фразу
/it - рaндомная запись с ithappens
/b - рaндомная запись с bash.im
/b id - запись с bash.im
/a - анекдот
/corona - covid stat
/corona [country] - covid stat by country
/corona top [help] - top 5 by new cases";

    api.execute(SendMessage::new(chat_id, help_msg)).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");

    let mut dispatcher = Dispatcher::new(api.clone());
    dispatcher.add_handler(handle_start);
    dispatcher.add_handler(handle_stop);
    dispatcher.add_handler(handle_corona);
    dispatcher.add_handler(handle_anekdot);
    dispatcher.add_handler(handle_bashorg);
    dispatcher.add_handler(handle_ithappens);
    dispatcher.add_handler(handle_mem);
    dispatcher.add_handler(handle_me);
    dispatcher.add_handler(handle_help);

    LongPoll::new(api, dispatcher).run().await;
}
