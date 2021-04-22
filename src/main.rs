extern crate tokio;
extern crate serde;
use serde::Deserialize;
use reqwest::Error;
use std::fs;
#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdout, Write};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

#[derive(Deserialize, Debug, Clone)]
struct Fields 
{
    item_id : String,
    item_name : String,
    brand_name : String,
    nf_calories: f32,
    nf_total_fat: f32,
    nf_serving_size_qty : f32,
    nf_serving_size_unit : String,
//    nf_total_carbohydrates : f32
}

#[derive(Deserialize, Debug, Clone)]
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

fn move_through_menu(stuff: &Vec<Hits>)
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let mut menu = "use left and right arrow keys to move through menu\n\r".to_owned();
    menu.push_str(&item_to_string(&stuff[0]));
    let max = stuff.len();
    let mut index = 0;
    let mut items: Vec<Hits> = Vec::new();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(menu))
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
                if index < max - 1
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
                           code : KeyCode::Enter,
                           ..
                       }) => {
                items.push(stuff[index].clone());
                let mut temp = "ADDED CURRENT ITEM TO LIST\n\r".to_owned();
                temp.push_str(&item_to_string(&stuff[index]));
                execute!(stdout, Clear(ClearType::All), Print(temp), cursor::MoveTo(0,0)).unwrap();
            },
            Event::Key(KeyEvent
                       {
                            code : KeyCode::Char('c'),
                            modifiers : KeyModifiers::CONTROL,
                       }) => {
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();

                break
            },
            Event::Key(KeyEvent
                       {
                            code : KeyCode::Char('i'),
                            ..
                       }) => {
                move_through_menu(&items);
            },

            _ => (),
        }
    }

    disable_raw_mode().unwrap();
}

fn print_item(_index : &usize, i : &Hits)
{
    let mut stdout = stdout();
    let holder = item_to_string(i);
    execute!(stdout, Clear(ClearType::All), Print(holder)).unwrap(); 
}

fn item_to_string(i : &Hits) -> String
{
    return format!(
    "Item Name : {}\n\r\
    Brand Name : {}\n\r\
    Serving Size qty : {}\n\r\
    Serving Size unit : {}\n\r\
    Calories : {}\n\r\
    Total Fat : {}",
    i.fields.item_name,
    i.fields.brand_name,
    i.fields.nf_serving_size_qty,
    i.fields.nf_serving_size_unit,
    i.fields.nf_calories,
    i.fields.nf_total_fat
    );
}

#[tokio::main]
async fn main()  -> Result<(), Error>
{
    let mut key = fs::read_to_string("nutrikey.txt").unwrap().trim().to_string().clone();
    const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'`');
    key = key.trim().to_string();
    let client = reqwest::Client::new();
    loop
    {
        let mut food = String::new();
        println!("Enter a food you would like to search: ");
        std::io::stdin().read_line(&mut food).unwrap();
        if food == "stop\n" || food == "s\n"
        {break}
        let after = utf8_percent_encode(&food, FRAGMENT).to_string();
        println!("before: {}\nafter: {}", food, after);
        let res = client
        .get(format!("https://nutritionix-api.p.rapidapi.com/v1_1/search/{}",after))
        .header("x-rapidapi-key" , &key)
        .header("x-rapidapi-host" ,  "nutritionix-api.p.rapidapi.com")
        .header("useQueryString", "true")
        .query(&[("fields", "item_id,item_name,brand_name,nf_calories,nf_total_fat,nf_serving_size_qty,nf_serving_size_unit")])
        .send()
        .await?;
//    let text = res.text().await?;
//   println!("{:?}", text);
    let json: Data = res.json().await?;
    move_through_menu(&json.hits);
    }
        Ok(())
}
