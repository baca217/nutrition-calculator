extern crate tokio;
extern crate serde;
use serde::Deserialize;
use reqwest::Error;
use std::env;
use std::fs;
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdin, stdout, Read, Write};

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
//    nf_total_carbohydrates : f32
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

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn move_through_menu(stuff: Vec<Hits>)
{
    let mut stdout = stdout();
    pause();
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
                if index < MAX - 1
                {
                    index += 1;
                    print_item(&index, &stuff[index])
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
                    print_item(&index, &stuff[index])
                }
            },
            Event::Key(KeyEvent
                       {
                            code : KeyCode::Char('c'),
                            modifiers : KeyModifiers::CONTROL,
                       }) => {
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();

                break
            },
            _ => (),
        }
    }

    disable_raw_mode().unwrap();
}

fn print_item(index : &usize, i : &Hits)
{
    let mut stdout = stdout();

    let holder = format!(
    "Number : {} \n\r\
    index : {}\n\r\
    type : {}\n\r\
    id : {}\n\r\
    score : {}\n\r\
    item_id : {}\n\r\
    item_name : {}\n\r\
    brand_name : {}\n\r\
    nf_serving_size_qty : {}\n\r\
    nf_serving_size_unit : {}\n\r\
    nf calories : {}\n\r\
    nf total fat : {}",
    index + 1,
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

    let mut key = fs::read_to_string("nutrikey.txt").unwrap();
    key = key.trim().to_string();
    let client = reqwest::Client::new();
    let res = client
        .get("https://nutritionix-api.p.rapidapi.com/v1_1/search/cheddar%20cheese")
        .header("x-rapidapi-key" , key)
        .header("x-rapidapi-host" ,  "nutritionix-api.p.rapidapi.com")
        .header("useQueryString", "true")
        .query(&[("fields", "item_id,item_name,brand_name,nf_calories,nf_total_fat,nf_serving_size_qty,nf_serving_size_unit")])
        .send()
        .await?;
//    let text = res.text().await?;
//   println!("{:?}", text);
    let json: Data = res.json().await?;
    println!("{:?}", json.hits[0]);
    move_through_menu(json.hits);
    Ok(())
}
