extern crate tokio;
extern crate serde;
use serde::Deserialize; //unpacking api call values
use reqwest::Error; //error for reqest crate
use std::fs;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc, Data};
#[macro_use]
extern crate crossterm;
use crossterm::cursor; //tracking the cursor
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers}; //catching input
use crossterm::style::Print; //cross platform print
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}; //cross platform types
use std::io::{stdout, stdin, Read, Write};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS}; //for encoding string to api urls
mod tools;

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
struct SearchData 
{
    total_hits: u32,
    max_score: f32,
    hits: Vec<Hits>,
}

#[derive(Clone, Data)]
struct Counter(i32);

fn move_through_menu(stuff: &Vec<Hits>)
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    let mut menu = "use left and right arrow keys to move through menu\n\r".to_owned();
    menu.push_str(&item_to_string(&stuff[0]));
    let max = stuff.len();
    let mut index = 0;
    let mut items: Vec<Hits> = Vec::new();

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(&menu))
        .unwrap();

    loop
    {
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();

        match read().unwrap()
        {
            Event::Key(KeyEvent //move right through the menu
                       {
                           code : KeyCode::Right,
                           ..
                       }) => {
                if index < max - 1
                {
                    index += 1;
                    print_item(&index, &stuff[index], None)
                }
            },
            Event::Key(KeyEvent //move left through the menu
                       {
                           code : KeyCode::Left,
                           ..
                       }) => {
                if index > 0
                {
                    index -= 1;
                    print_item(&index, &stuff[index], None)
                }
            },
            Event::Key(KeyEvent //add current item to list
                       {
                           code : KeyCode::Enter,
                           ..
                       }) => {
                items.push(stuff[index].clone());
                let mut temp = "ADDED CURRENT ITEM TO LIST\n\r".to_owned();
                temp.push_str(&item_to_string(&stuff[index]));
                execute!(stdout, Clear(ClearType::All), Print(temp), cursor::MoveTo(0,0)).unwrap();
            },
            Event::Key(KeyEvent //break from the function
                       {
                            code : KeyCode::Char('c'),
                            modifiers : KeyModifiers::CONTROL,
                       }) => {
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();

                break
            },
            Event::Key(KeyEvent //checkout the items that were saved
                       {
                            code : KeyCode::Char('i'),
                            ..
                       }) => {
                items = move_through_items(items);
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(&menu))
                    .unwrap();
            },
            _ => (), //catch all, do nothing
        }
    }

    disable_raw_mode().unwrap();
}


fn move_through_items(mut items: Vec<Hits>) -> Vec<Hits>
{
    let mut stdout = stdout();
    enable_raw_mode().unwrap();
    if items.len() == 0
    {
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), 
                 Print("nothing in the saved list!\r\n")).unwrap();
        tools::pause();
        return items;
    }
    let menu = "use left and right arrow keys to move through items";
    let mut max = items.len();
    let mut index = 0;
    let _no_mods = KeyModifiers::empty();

    print_item(&0_usize, &items[0], Some(menu.to_string()));

    loop
    {
        if max == 0
        {
            execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), 
                     Print("nothing in the saved list!\r\n")).unwrap();
            tools::pause();
            return items;
        }

        print_item(&index, &items[index], None);

        match read().unwrap()
        {
            Event::Key(KeyEvent //move right through menu
                       {
                           code : KeyCode::Right,
                           ..
                       }) => {
                if index < max - 1
                {
                    index += 1;
                }
            },
            Event::Key(KeyEvent //move left through menu
                       {
                           code : KeyCode::Left,
                           ..
                       }) => {
                if index > 0
                {
                    index -= 1;
                }
            },
            Event::Key(KeyEvent //break from function
                       {
                            code : KeyCode::Char('c'),
                            modifiers : KeyModifiers::CONTROL,
                       }) => {
                execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0)).unwrap();
                break
            },
            Event::Key(KeyEvent //delete the curent item
                       {
                            code : KeyCode::Char('d'),
                            ..
                       }) => {
                loop
                {
                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), 
                             Print("are you sure you would like to remove the food?")).unwrap();
                    disable_raw_mode().unwrap();
                    let ans = tools::get_input();
                    enable_raw_mode().unwrap();
                    if ans == "yes\n" || ans == "y\n" && items.len() != 0
                    {
                        items.remove(index);
                        if index > 0
                        {index -= 1;}
                        max -= 1;
                        break;
                    }
                    else if ans == "no\n" || ans == "n\n"
                    {
                        break
                    }
                }
            },
            _ => (),
        }
    }
    return items;
}


fn print_item(_index : &usize, i : &Hits, msg : Option<String>)
{
    let mut stdout = stdout();
    let mut holder = item_to_string(i);
    match msg
    {
        Some(val) => holder = format!("{}\n\r{}", val, holder),
        None => (),
    }
    
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0,0), Print(holder)).unwrap(); 
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

fn test_display() -> Result<(), PlatformError>
{
    let main_window = WindowDesc::new(ui_builder)
        .title("Hello, Druid!")
        .window_size((200.0, 100.0));

    let data: Counter = Counter(0);

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<Counter>
{
    let text = LocalizedString::new("hello-counter")
        .with_arg("count", |data: &Counter, _env| (*data).0.into());
    let label = Label::new(text).padding(5.0).center();

    let button_plus = Button::new("+1")
        .on_click(|_ctx, data: &mut Counter, _env| (*data).0 += 1)
        .padding(5.0);

    let button_minus = Button::new("-1")
        .on_click(|_ctx, data: &mut Counter, _env| (*data).0 -= 1)
        .padding(5.0);

    let flex = Flex::row()
        .with_child(button_plus)
        .with_spacer(1.0)
        .with_child(button_minus);

    Flex::column()
        .with_child(label)
        .with_child(flex)
}


#[tokio::main]
async fn main()  -> Result<(), Error>
{
    test_display();
    std::process::exit(0);

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
    let json: SearchData = res.json().await?;
    move_through_menu(&json.hits);
    }
        Ok(())
}
