use std::io;
use std::collections::HashMap;

const QUOT: char = '"';



fn main() {
    println!("\n{headline:>width$}\n", headline = "---Wellcome to sky-armor---", width=60);
    let help_com = String::from("help");
    let add_empl_com = String::from("add_empl");
    let rem_empl_com = String::from("rem_empl");
    let rebase_empl_com = String::from("rebase_empl");
    let show_empl_dep_com = String::from("show_empl_dep");

    'menu: loop{
        let command = enter_command();
        match command{
            Option::Some(help_com) => {help(); continue 'menu},
            String::from(add_empl_com)
            String::from(rem_empl_com)
            String::from(rebase_empl_com)
            String::from(show_empl_dep_com)
            _=> println!{("There is no same command. Please, enter you command again or use {0}help{0}", QUOT); continue 'menu}
        }
    }

}

fn enter_command() -> Option<String>{

    let mut user_command = String::new();
    print!("Enter command (for inquiry enter {0}help{0}): ", QUOT);
    io::stdin().read_line(&mut user_command).expect("Something wrong with OS std:in");
    
    user_command.trim().to_string();

    match user_command.is_empty(){
        true => Option::None,
        false => Option::Some(user_command),
    }
}

fn help(){
    println!("\n{headline:>width$}\n", headline = "---Available commands---", width=58);
    print!("{el1:>width$} - add new employee;\n
            {el2:>width$} - remove an employee;\n
            {el3:>width$} - rebase an employee to another department;\n
            {el4:>width$} - find an employee from own department;\n", 
            el1 = "add_empl", el2 = "rem_empl", el3 = "rebase_empl", el4 = "show_empl_dep", width=20);
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