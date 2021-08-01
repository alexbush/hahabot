use rusqlite::{params, Connection, Result};

const PATH: &str = "./memory.sqlite";

fn rename(id: i64) -> String {
    let s = format!("{}", id);
    s.replace("-", "minus_")
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

pub struct Memo {
    pub chat_id : i64,
    table       : String,
    pub message : Option<String>,
}

impl Memo {
    pub fn new(chat_id: i64) -> Self {
        Self {
            chat_id,
            table      : rename(chat_id),
            message    : None,
        }
    }
    pub fn set_message(&mut self, message: &str) -> &Self {
        self.message = Some(message.to_owned());
        self
    }

    #[allow(clippy::unit_arg)]
    pub fn get(&mut self, id: Option<i64>) -> Result<String> {
        let db = Connection::open(&PATH)?;
        let mut msg = String::new();
        if id.is_none() {
            let query = format!(
                "SELECT message FROM `{}` ORDER BY RANDOM() LIMIT 1", self.table
            );
            let mut st = db.prepare(query.as_str())?;
            st.query_row(params![], |r| { Ok(msg = r.get(0)?) })?;
        } else {
            let query = format!(
                "SELECT message FROM `{}` WHERE id = ?1", self.table
            );
            let mut st = db.prepare(query.as_str())?;
            st.query_row(params![id], |r| { Ok(msg = r.get(0)?) })?;
        }

        Ok(msg)
    }
    pub fn remove(&mut self, id: i64) -> Result<bool> {
        let db = Connection::open(&PATH)?;
        let query = format!(
            "DELETE FROM `{}` WHERE id = ?1", self.table
        );
        db.execute(query.as_str(), params![id])?;

        Ok(true)
    }
    pub fn save(self) -> Result<()> {
        let db = Connection::open(&PATH)?;

        let query = format!("INSERT INTO `{}` (message, author, created)
                VALUES(?, 'Anonymous', datetime('now'))",
                self.table,
        );
        db.execute(query.as_str(), params![self.message])?;

        Ok(())
    }
}
