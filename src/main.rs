// #![allow(unused)]
use carapax::types::Message;
use carapax::{
    longpoll::LongPoll,
    methods::SendMessage,
    types::ChatId, 
    Api, App, Context, ExecuteError, Ref,
};
use dotenv::dotenv;
use std::env;
use postgres::{Client, NoTls, Error};

struct Sorted {
    item: String,
    price: i64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let token = env::var("CARAPAX_TOKEN").expect("CARAPAX_TOKEN is not set");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    LongPoll::new(api, app).run().await
}

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Message) -> Result<(), ExecuteError> {
    let content = message.get_text().unwrap();
    let string_slice = &content.data[..=5];
    let second_part = &content.data[6..];
    match string_slice {
        "/start" => api.execute(SendMessage::new(chat_id.clone(), "Received /start")).await?,
        "/dbadd" => api.execute(SendMessage::new(chat_id.clone(), "Received /dbadd")).await?,
        "/print" => api.execute(
            SendMessage::new(
                chat_id.clone(), format!("{:?}", some(second_part))
            )).await?,
        _ => api.execute(SendMessage::new(chat_id.clone(), "Error")).await?,
    };
    
    Ok(())
}

fn some(slice: &str) -> Result<String, Error> {
    sorted();
    let a = format!("fn some: {}", slice);
    Ok(a)
}

async fn sorted() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls
    )?;
    
    for row in client.query 
    ("SELECT item, COUNT(item) AS goods 
    FROM b_store GROUP BY id ORDER BY goods DESC", &[])? {
        let (item, price): (Option<String>, Option<i64>) = (row.get(0), row.get(1));
        
        if item.is_some() && price.is_some() {
            let sorted_db = Sorted {
                item: item.unwrap(),
                price: price.unwrap(),
        };
            println!("{} {}", sorted_db.item, sorted_db.price);
            
        }
    }
        
    Ok(())
}