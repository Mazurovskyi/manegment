use std::io;
use std::collections::HashMap;
use std::mem::discriminant;

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

#[derive(Debug)]
enum Sex{
    male,
    feemale,
    russian,
    other
}

#[derive(Debug)]
enum Department{
    developer,
    admin,
    engineering,
    accountant
}

#[derive(Debug)]
struct Employee{
    name: String,
    sex: Sex,
    vaccine: bool,
    education: String,
    age: u8,
    department: Department,
    chief: bool
}

fn main() {
    println!("\n{headline:>width$}\n", headline = "---Wellcome to sky-armor---", width=60);
    let com = format!("Enter command (to get instructions enter {0}help{0}): ", QUOT);


    let mut employee_vect: Vec<Employee> = Vec::new();


    'menu: loop{
        let command = enter_string(&com).turn_command();
        match command{
            Command::help => {help(); continue 'menu},
            Command::add_empl => add_empl(&mut employee_vect),
            Command::rem_empl => rem_empl(),
            Command::rebase_empl => rebase_empl(),
            Command::show_empl_dep => show_empl_dep(),
            Command::unknown => {println!("\nUnknown command! Try to enter one more or use {0}help{0} to get instructions", QUOT); continue 'menu },
            Command::empty => {println!("\nEnter the command or use {0}help{0} to get instructions", QUOT); continue 'menu }
        }
    }

}


fn add_empl(employee_vect: &mut Vec<Employee>){
    println!("      Adding new employee: ");

    let mut val: bool = true;
    
    let mut new_employee = new_employee();

    for worker in employee_vect{
        if discriminant(&worker.department) == discriminant(&new_employee.department){
            if worker.chief == true{                                
                val = false
            }                           
        }
    }

    //employee_vect.push(new_employee); 

    match val{
        true => {
            'enter_chief: loop{
                let message = format!("Would you like to make {0} a chief of department? (Enter {1}yes{1} or {1}no{1})", new_employee.name, QUOT);
                match enter_string(&message){
                    Option::Some(emp_chief) => {
                        match emp_chief.trim(){

                            "yes" => {
                                new_employee.chief = true; 
                                employee_vect.push(new_employee); 
                                break 'enter_chief
                            },
                            "no" => {
                                employee_vect.push(new_employee); 
                                break 'enter_chief
                            },
                            _=> {
                                println!("Please, enter {0}yes{0} or {0}no{0}!", QUOT); 
                                continue 'enter_chief
                            }

                        }
                    },
                    Option::None => {println!("Please, enter something"); continue 'enter_chief}
                }
            }
        }
        false => employee_vect.push(new_employee)
    }
    
}




fn new_employee()-> Employee{
    let mut sex_counter = 0;
    Employee{
        name: {
            'enter_name: loop{
                match enter_string("Name: "){
                    Option::Some(emp_name) => break 'enter_name emp_name,
                    Option::None => {println!("Enter name!"); continue 'enter_name}
                }
            }
        },
        sex:{
            'enter_sex: loop{
                match enter_string("Sex: "){
                    Option::Some(emp_sex) => {
                        match emp_sex.trim(){
                            "male" => break 'enter_sex Sex::male,
                            "feemale" => break 'enter_sex Sex::feemale,
                            "russian" => break 'enter_sex Sex::russian,
                            _=> {
                                if sex_counter == 3{
                                    println!("You are a fucking foodie!"); break 'enter_sex Sex::other
                                }
                                else{
                                    sex_counter+=1;
                                    println!("Enter the normal sex!"); continue 'enter_sex                                 
                                }                               
                            }
                        }
                    },
                    Option::None => {println!("Enter sex!"); continue 'enter_sex}
                }
            }
        },
        vaccine:{
            'enter_vaccine: loop{
                match enter_string("vaccine (true or false): "){
                    Option::Some(emp_vaccine) => {
                        match emp_vaccine.trim(){
                            "true"=> break 'enter_vaccine true,
                            "false" => break 'enter_vaccine false,
                            _=> {println!("You need to enter {0}true{0} or {0}false{0}", QUOT); continue 'enter_vaccine}
                        }

                    }
                    Option::None => {println!("Enter {0}true{0} or {0}false{0}!", QUOT); continue 'enter_vaccine}
                }
            }
        },
        education:{
            'enter_education: loop{
                match enter_string("Education: "){
                    Option::Some(emp_education) => break 'enter_education emp_education,
                    Option::None => {println!("Enter something about education!"); continue 'enter_education}
                }
            }
        },
        age:{                                               
            'enter_age: loop{
                match enter_string("Education: "){
                    Option::Some(emp_ege) => {
                        match emp_ege.trim().parse(){                                       //компилятор знает к какому типу парсить????
                            Result::Ok(emp_ege) => break 'enter_age emp_ege,
                            Result::Err(message) => {println!("Enter the quess! {}", message); continue 'enter_age}
                        }
                    },
                    Option::None => {println!("Enter the age!"); continue 'enter_age}
                }
            }
        },
        department:{
            'enter_department: loop{
                match enter_string("Department (developer / admin / engineering / accountant): "){
                    Option::Some(emp_dep) => {
                        match emp_dep.trim(){
                            "developer" => {break 'enter_department Department::developer},
                            "admin" => {break 'enter_department Department::admin},
                            "engineering" => {break 'enter_department Department::engineering},
                            "accountant" => {break 'enter_department Department::accountant},
                            _=> {
                                    println!("Enter one of the departments!"); continue 'enter_department                                                          
                            }
                        }
                    },
                    Option::None => {println!("The employe couldn't be without own department!"); continue 'enter_department}
                }
            }
        },
        chief: false
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

fn enter_string(parametr: &str) -> Option<String>{
    let mut user_data = String::new();
    println!("{}", parametr);
    io::stdin().read_line(&mut user_data).expect("Something wrong with OS std:in");

    user_data = user_data.trim().to_string();

    match user_data.is_empty(){
        true => Option::None,
        false => Option::Some(user_data),
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