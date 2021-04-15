extern crate tokio;
extern crate serde;
use serde::Deserialize;
use reqwest::Error;
use std::env;
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};

#[derive(Deserialize, Debug)]
struct Fields 
{
    item_id : String,
    item_name : String,
    brand_name : String,
    nf_calories: f32,
    nf_total_fat: f32,
    nf_serving_size_qty : u32,
    nf_serving_size_unit : String,
}

#[derive(Deserialize, Debug)]
struct Hits 
{
    _index: String,
    _type : String,
    _id : String,
    _score : f32,
    fields : Fields,
}

#[derive(Deserialize, Debug)]
struct Data 
{
    total_hits: u32,
    max_score: f32,
    hits: Vec<Hits>,
}

fn key_test()
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(r#"ctrl + q to exit, ctrl + x 
        to print "Hello World", alt + t to print "crossterm is cool""#))
        .unwrap();

    loop
    {
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();

        match read().unwrap()
        {
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Char('h'),
                           modifiers : KeyModifiers::CONTROL,

                       }) => execute!(stdout, Clear(ClearType::All), Print("Hello World!")).unwrap(),
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Char('t'),
                           modifiers : KeyModifiers::ALT,

                       }) => execute!(stdout, Clear(ClearType::All), Print("crossterm is cool")).unwrap(),
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Char('q'),
                           modifiers : KeyModifiers::CONTROL,

                       }) => break,
            _ => (),
        }
    }

    disable_raw_mode().unwrap();
}

#[tokio::main]
async fn main()  -> Result<(), Error>
{
    key_test();
    //-------------------------------------------------------------------
    let args: Vec<String> = env::args().collect();

    let key = &args[1];
    let client = reqwest::Client::new();
    let res = client
        .get("https://nutritionix-api.p.rapidapi.com/v1_1/search/cheddar%20cheese")
        .header("x-rapidapi-key" , key)
        .header("x-rapidapi-host" ,  "nutritionix-api.p.rapidapi.com")
        .header("useQueryString", "true")
        .query(&[("fields", "item_name,item_id,brand_name,nf_calories,nf_total_fat")])
        .send()
        .await?;

    let json: Data = res.json().await?;
    for i in json.hits{
        println!("index : {}", i._index);
        println!("type : {}", i._type);
        println!("id : {}", i._id);
        println!("score : {}", i._score);
        println!("item_id : {}", i.fields.item_id);
        println!("item_name : {}", i.fields.item_name);
        println!("brand_name : {}", i.fields.brand_name);
        println!("nf_serving_size_qty : {}", i.fields.nf_serving_size_qty);
        println!("nf_serving_size_unit : {}", i.fields.nf_serving_size_unit);
        println!("nf calories : {}", i.fields.nf_calories);
        println!("nf total fat : {}", i.fields.nf_total_fat);
        println!("");
    }
    Ok(())
}
//    let string = res.text().await?;
//    println!("Text: \n{}", string);


