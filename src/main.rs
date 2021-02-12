use carapax::{
    Api,
    Config,
    Dispatcher,
    ExecuteError,
    HandlerResult,
    handler,
    longpoll::LongPoll,
    methods::{ SendMessage, DeleteMessage},
    types::{Command, ParseMode},
};
use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use tokio::sync::Mutex;
use std::sync::Arc;

mod corona;
mod memory;
mod sources;

struct Context {
    api: Api,
    count: Arc<Mutex<i32>>,
}

#[handler(command = "/start")]
async fn handle_start(context: &Context, command: Command) -> Result<HandlerResult, ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    match memory::create_table(chat_id) {
        Ok(_) => log::info!("table {} created", chat_id),
        Err(why) => log::warn!("can't create table {}: {}", chat_id, why)
    }
    let method = SendMessage::new(chat_id, "Всем чьмоки в этом чате!");
    context.api.execute(method).await?;
    Ok(HandlerResult::Stop)
}

#[handler(command = "/me")]
async fn handle_me(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let message = command.get_message();
    let chat_id = message.get_chat_id();

    println!("{:#?}", message);


    let me: String = match message.get_user() {
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

    match context.api.execute(DeleteMessage::new(chat_id, message.id)).await {
        Ok(_) => (),
        Err(why) => println!("{}", why),
    }

    let answer = match message.reply_to.clone() {
        None => format!("{} {}", me, user_message),
        Some(reply) => match reply.get_user() {
            Some(user) => match user.username.clone() {
                Some(username) => format!("@{}, {} {}", username, me, user_message),
                None => format!("{}, {} {}", user.first_name.clone(), me, user_message),
            },
            None => format!("{} {}", me, user_message),
        }
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}

#[handler(command = "/m")]
async fn handle_mem(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let message  = command.get_message();
    let chat_id  = message.get_chat_id();
    let args     = command.get_args();
    let mut memo = memory::Memo::new(chat_id);

    match message.get_text() {
        None => return Ok(()),
        Some(text) => {
            if !text.data.starts_with("/m") {
                return Ok(());
            }
        }
    };

    let answer = if args.is_empty() {
        match message.reply_to.clone() {
            None => {
                match context.api.execute(DeleteMessage::new(chat_id, message.id)).await {
                    Ok(_) => (),
                    Err(why) => println!("{}", why),
                }

                memo.get(None).unwrap_or("not found".to_string())
            },
            Some(reply) => match reply.get_text() {
                None => "forward message not found".to_string(),
                Some(text) => {
                    memo.set_message(text.data.clone());
                    match memo.save() {
                        Ok(_) => "saved".to_string(),
                        Err(err) => {
                            log::warn!("Cant save message: {}", err);
                            "[:||||:]".to_string()
                        }
                    }
                },
            },
        }
    } else {
        match args[0].parse::<i64>() {
            Ok(id) => memo.get(Some(id)).unwrap_or("not found".to_string()),
            Err(_) => {
                match message.get_text() {
                    None => "".to_string(),
                    Some(text) => {
                        memo.set_message(text.data.clone());
                        match memo.save() {
                            Ok(_) => "saved".to_string(),
                            Err(err) => {
                                log::warn!("Cant save message: {}", err);
                                "[:||||:]".to_string()
                            }
                        }
                    },
                }
            }
        }
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;
    Ok(())
}

#[handler(command = "/stop")]
async fn handle_stop(context: &Context, command: Command) -> Result<(), ExecuteError> {
    log::info!("handle /stop command\n");
    let message = command.get_message();
    let chat_id = message.get_chat_id();
    let method = SendMessage::new(chat_id, "Cant stop me now!");
    context.api.execute(method).await?;
    Ok(())
}

#[handler(command = "/it")]
async fn handle_ithappens(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let answer = match sources::ithappens().await {
        Ok(body) => body,
        Err(_) => "not found".to_string(),
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;
    Ok(())
}

#[handler(command = "/a")]
async fn handle_anekdot(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let answer = match sources::anekdot().await {
        Ok(body) => body,
        Err(_) => "not found".to_string(),
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}

#[handler(command = "/corona")]
async fn handle_corona(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args    = command.get_args();
    let corona  = corona::Corona::new(args.to_vec()).await;
    let answer: String = match corona.get().await {
        Ok(a) => a,
        Err(why) => {
            log::error!("Error while getting covid info: {}", why);
            format!("Meow, I have paws ^_^")
        },
    };

    context.api.execute(SendMessage::new(chat_id, answer)
        .parse_mode(ParseMode::MarkdownV2)
    ).await?;

    Ok(())
}

#[handler(command = "/b")]
async fn handle_bashorg(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();
    let args = command.get_args();

    let answer = if args.is_empty() {
        match sources::bash(0).await {
            Ok(body) => body,
            Err(_) => "not found".to_string(),
        }
    } else {
        match sources::bash(args[0].parse::<u64>().unwrap_or(0)).await {
            Ok(body) => body,
            Err(_) => "not found".to_string(),
        }
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}

#[handler(command = "/count")]
async fn handle_count(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let cnt = context.count.lock().await;

    context.api.execute(SendMessage::new(chat_id, format!("count: {}", cnt))).await?;

    Ok(())
}


#[handler(command = "/dtp")]
async fn handle_dtp(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let answer = match sources::dtp().await {
        Ok(body) => body,
        Err(_) => "not found".to_string(),
    };

    context.api.execute(SendMessage::new(chat_id, answer)).await?;

    Ok(())
}


#[handler(command = "/h")]
async fn handle_help(context: &Context, command: Command) -> Result<(), ExecuteError> {
    let chat_id = command.get_message().get_chat_id();

    let mut cnt = context.count.lock().await;
    *cnt += 1;
    log::info!("count: {}", cnt);

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
/dtp - ДТП по РФ
/corona - covid stat
/corona vaccine - vaccine info
/corona [country] - covid stat by country
/corona top [help] - top 5 by new cases";

    context.api.execute(SendMessage::new(chat_id, help_msg)).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TGRS_TOKEN").expect("TGRS_TOKEN is not set");
    let config = Config::new(token);
    let api = Api::new(config).expect("Failed to create API");

    let count = Arc::new(Mutex::new(0));
//     let mut dispatcher = Dispatcher::new(api.clone());
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
}
