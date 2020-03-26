use rusqlite::{params, Connection, Result};
use time::Timespec;
use log;

#[derive(Debug)]
pub struct Message {
    pub id: i64,
    pub message: String,
    pub author: String,
    pub created: Timespec,
}

const PATH: &str = "./memory.sqlite";

fn rename(id: i64) -> String {
    let s = format!("{}", id);
    let table = s.replace("-", "minus_");
    table
}

pub fn create_table(chat_id: i64) -> Result<()> {
    let db = Connection::open(&PATH)?;

    let table = rename(chat_id);

    log::info!("create table");
    let query = format!(
        "CREATE TABLE IF NOT EXISTS `{}` (
            id      integer, 
            message text, 
            author  varchar, 
            created datetime, 
            PRIMARY KEY (id))",
        table
    );

    match db.execute( query.as_str(), params![]) {
        Ok(_) => {
            let index = format!(
                "CREATE UNIQUE INDEX `message_{}` ON `{}` (message)", table, table);
            
            match db.execute( index.as_str(), params![]) {
                Ok(_) => Ok(()),
                Err(why) => Err(why)
            }
        },
        Err(why) => Err(why)
    }
}

pub fn save(chat_id: i64, me: &Message) -> Result<()> {
    let db = Connection::open(&PATH)?;

    let table = rename(chat_id);

    let query = format!("INSERT INTO `{}` (message, author, created) 
                VALUES('{}', '{}', datetime('now'))",
                    table,
                    me.message,
                    me.author,
                );
    db.execute(query.as_str(), params![])?;

    Ok(())
}

pub fn get(chat_id: i64, id: i64) -> Result<Message> {
    let db = Connection::open(&PATH)?;
    log::info!("try to get {} from {}", id, chat_id);
    
    let table = rename(chat_id);

    let query = format!(
        "SELECT id, message, created, author
        FROM `{}` WHERE id = ?1", 
        table
    );

    let mut st = db.prepare(query.as_str())?;

    let row = st.query_row(params![id], |r| {
        Ok(Message {
            id: r.get(0)?,
            message: r.get(1)?,
            created: r.get(2)?,
            author: r.get(3)?,
        })
    })?;

    Ok(row)
}

pub fn get_random(chat_id: i64) -> Result<Message> {
    let db = Connection::open(&PATH)?;
    log::info!("try to get random message from {}", chat_id);
    
    let table = rename(chat_id);

    let query = format!(
        "SELECT id, message, created, author
        FROM `{}`
        ORDER BY RANDOM() LIMIT 1", table
    );

    log::info!("{}", query);

    let mut st = db.prepare(
        query.as_str()
    )?;

    let row = st.query_row(params![], |r| {
        Ok(Message {
            id: r.get(0)?,
            message: r.get(1)?,
            created: r.get(2)?,
            author: r.get(3)?,
        })
    })?;

    Ok(row)
}
