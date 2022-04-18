use std::io;
use std::collections::HashMap;

const QUOT: char = '"';

enum Command{
    help,
    add_empl,
    rem_empl,
    rebase_empl,
    show_empl_dep,
    empty,
    unknown
}

enum Option<T>{
    Some(T),
    None
}

fn main() {
    println!("\n{headline:>width$}\n", headline = "---Wellcome to sky-armor---", width=60);

    


    'menu: loop{
        let command = enter_data().turn_command();
        match command{
            Command::help => {help(); continue 'menu},
            Command::add_empl => add_empl(),
            Command::rem_empl => rem_empl(),
            Command::rebase_empl => rebase_empl(),
            Command::show_empl_dep => show_empl_dep(),
            Command::unknown => {println!("\nUnknown command! Try to enter one more or use {0}help{0} to get instructions", QUOT); continue 'menu },
            Command::empty => {println!("\nEnter the command or use {0}help{0} to get instructions", QUOT); continue 'menu }
        }
    }

}



impl Option<String>{

    fn turn_command(self) -> Command{

        match self{
            Option::Some(user_value) => {

                match user_value.trim(){
                    "help" => Command::help,               
                    "add_empl" => Command::add_empl,
                    "rem_empl" => Command::rem_empl,
                    "rebase_empl" => Command::rebase_empl,
                    "show_empl_dep" => Command::show_empl_dep,              
                    _=> Command::unknown
                }
            },
            Option::None => Command::empty
        }
    }
}

fn enter_data() -> Option<String>{
    let mut user_command = String::new();
    println!("Enter command (for inquiry enter {0}help{0}): ", QUOT);
    io::stdin().read_line(&mut user_command).expect("Something wrong with OS std:in");

    user_command = user_command.trim().to_string();

    match user_command.is_empty(){
        true => Option::None,
        false => Option::Some(user_command),
    }
}

fn help(){
    println!("\n{headline:>width$}", headline = "---Available commands---", width=59);
    println!("{el1} - add new employee;\r\n{el2} - remove an employee;\r\n{el3} - rebase an employee to another department;\r\n{el4} - find an employee from own department;\r\n{el5} - get information about avallible commands;\r\n", 
            el1 = "add_empl", 
            el2 = "rem_empl", 
            el3 = "rebase_empl", 
            el4 = "show_empl_dep", 
            el5 = "help");
}


fn add_empl(){
println!("add_empl funktion");
}


fn rem_empl(){
    println!("rem_empl funktion");
}

fn rebase_empl(){
    println!("rebase_empl funktion");
}


fn show_empl_dep(){
    println!("show_empl_dep funktion");
}




//------------------------------------------------------------------------------------------------------------------
//                                                      OPTIONS

    /*
    println!("Hello, world!");
    println!("{number:>width$}", number=111, width=10); // абзац
    println!("{number:->width$}", number=1, width=10); // заполнение символами
    println!("My name is {0}, {1} {0}", "Bond", "James"); // поочередный вывод значений

    let s: char = '"';
    println!("Enter {0}text{0}", s);
    */