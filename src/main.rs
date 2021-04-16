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

fn move_through_menu(stuff: Vec<Hits>)
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let MENU = "use left and right arrow keys to move through menu";
    let MAX = stuff.len();
    let mut index = 0;

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(MENU))
        .unwrap();

    loop
    {
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();

        match read().unwrap()
        {
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Right,
                           ..
                       }) => {
                if index < MAX
                {
                    index += 1;
                    print_item(&stuff[index])
                }
                else
                {
                    println!("index is at {}, can't add 1", index);
                }
            },
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Left,
                           ..
                       }) => {
                if index > 0
                {
                    index -= 1;
                    print_item(&stuff[index])
                }
                else
                {
                    println!("index is at {}, can't subtract 1", index);
                }
            },
            _ => execute!(stdout, Clear(ClearType::All), Print(MENU)).unwrap(),
        }
    }

    disable_raw_mode().unwrap();
}


fn key_test()
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let MENU = r#"ctrl + q to exit, ctrl + x to print "Hello World", alt + t to print "crossterm is cool""#;

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(MENU))
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
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Up,
                           ..
                       }) => execute!(stdout, Clear(ClearType::All), Print("pressed up!!!")).unwrap(),
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Down,
                           ..
                       }) => execute!(stdout, Clear(ClearType::All), Print("pressed down!!!")).unwrap(),
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Right,
                           ..
                       }) => execute!(stdout, Clear(ClearType::All), Print("pressed right!!!")).unwrap(),
            Event::Key(KeyEvent
                       {
                           code : KeyCode::Left,
                           ..
                       }) => execute!(stdout, Clear(ClearType::All), Print("pressed left!!!")).unwrap(),
            _ => execute!(stdout, Clear(ClearType::All), Print(MENU)).unwrap(),
        }
    }

    disable_raw_mode().unwrap();
}

fn print_item(i : &Hits)
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    let holder = "";
    println!(
    "index : {}s\n
    type : {}s\n 
    id : {}s\n 
    score : {}s\n 
    item_id : {}s\n 
    item_name : {}s\n 
    brand_name : {}s\n 
    nf_serving_size_qty : {}s\n 
    nf_serving_size_unit : {}s\n 
    nf calories : {}s\n 
    nf total fat : {}s\n",
    i._index,
    i._type,
    i._id,
    i._score,
    i.fields.item_id,
    i.fields.item_name,
    i.fields.brand_name,
    i.fields.nf_serving_size_qty,
    i.fields.nf_serving_size_unit,
    i.fields.nf_calories,
    i.fields.nf_total_fat
    );

    execute!(stdout, Clear(ClearType::All), Print(holder)).unwrap(); 
}

#[tokio::main]
async fn main()  -> Result<(), Error>
{
    //key_test();
    //std::process::exit(1);
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
    move_through_menu(json.hits);
    Ok(())
}
//    let string = res.text().await?;
//    println!("Text: \n{}", string);


