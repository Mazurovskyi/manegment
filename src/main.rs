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

#[derive(Debug)]
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

#[derive(Debug)]
struct Data_to_rebase{
    name: String,
    department: Department
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
            Command::rem_empl => rem_empl(&mut employee_vect),
            Command::rebase_empl => rebase_empl(&mut employee_vect),
            Command::show_empl_dep => show_empl_dep(&employee_vect),
            Command::unknown => {println!("\nUnknown command! Try to enter again or use {0}help{0} to get instructions", QUOT); continue 'menu },
            Command::empty => {println!("\nEnter the command or use {0}help{0} to get instructions", QUOT); continue 'menu }
        }
    }

}


// ---main commands---

fn add_empl(employee_vect: &mut Vec<Employee>){
    println!("\n      Adding new employee: ");
    
    let mut new_employee = new_employee();

    let val = find_the_chief(employee_vect, &new_employee);

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

fn help(){
    println!("\n{headline:>width$}", headline = "---Available commands---", width=59);
    println!("{el1} - add new employee;\r\n{el2} - remove an employee;\r\n{el3} - rebase an employee to another department;\r\n{el4} - find an employee from own department;\r\n{el5} - get information about avallible commands;\r\n", 
            el1 = "add_empl", 
            el2 = "rem_empl", 
            el3 = "rebase_empl", 
            el4 = "show_empl_dep", 
            el5 = "help");
}

fn rem_empl(employee_vect: &mut Vec<Employee>){
    println!("\n      Removing the employee");
    'removing: loop{
        match enter_string("Enter the name of employee to remove:"){
            Option::Some(user_name) => {



                match find_the_name(employee_vect, &user_name){
                    Option::Some(worker) => {
                        delete_element(employee_vect, &worker);
                        break 'removing
                    },
                    Option::None => {
                        'removing_inner: loop{
                            let message = format!("There are no employees like {1}{0}{1}! Do you want to open the base? ({1}yes{1} or {1}no{1})", user_name, QUOT);
                            match enter_string(&message){
                                Option::Some(command) => {
                                    match command.trim(){
                                        "yes" => show_empl_dep(employee_vect),
                                        "no" => continue 'removing,
                                        _=> {println!("Please, enter {0}Yes{0} or {0}no{0}!", QUOT); continue 'removing_inner}
                                    }
                                },
                                Option::None => continue 'removing_inner
                            }
                            continue 'removing
                        }
                    }
                }




            }
            Option::None => {println!("Enter the name!"); continue 'removing}
        }
    }


    
}

fn show_empl_dep(employee_vect: &Vec<Employee>){
    println!("\n      Employee base\n");
    let message = format!("Enter the department name or {0}all{0} to see all employees:", QUOT);

    'show_empl: loop{
        match enter_string(&message){
            Option::Some(user_data) => {

                match user_data.trim(){
                    "developer" => {show_data(&Department::developer, employee_vect); break 'show_empl},
                    "admin" => {show_data(&Department::admin, employee_vect); break 'show_empl},
                    "engineering" => {show_data(&Department::engineering, employee_vect); break 'show_empl},
                    "accountant" => {show_data(&Department::accountant, employee_vect); break 'show_empl},
                    "all" => show_data_all(employee_vect),
                    _=> {println!("Please, choose one of the commands"); continue 'show_empl}
                }


            },
            Option::None => {println!("Enter the comand!"); continue 'show_empl}
        }
    }
}

// ---subsidiary commands---

fn find_the_chief(employee_vect: &mut Vec<Employee>, new_employee: &Employee)-> bool{
    let mut val: bool = true;
    
    for worker in employee_vect{      
        if discriminant(&worker.department) == discriminant(&new_employee.department){
            if worker.chief == true{                                
                val = false
            }                           
        }
    }
    val
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
                match enter_string("Age: "){      
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

fn find_the_name(employee_vect: &mut Vec<Employee>, user_name: &String) -> Option<String>{
let mut result: Option<String> = Option::None;

    for worker in employee_vect{
        if worker.name == *user_name{
            result = Option::Some(worker.name.clone())
        }
    }
    result
}

fn delete_element(employee_vect: &mut Vec<Employee>, worker: &String){
    let mut i = 0;

    let mut iter = employee_vect.iter();

    'internal: loop{
        match iter.next(){
            Some(value) => {

                if value.name == *worker{
                    break 'internal
                }  
                i+=1; 

            },
            None => break 'internal,
        }
    }

    employee_vect.swap_remove(i);
    println!("Successfully! Employee was removed\n");
}

fn show_data(department: &Department, employee_vect: &Vec<Employee>){

    println!("      {:?} department:\n", department);

    let mut employee_list = Vec::new();
    for worker in employee_vect{
        if discriminant(&worker.department) == discriminant(department){
            employee_list.push(worker);
        }
    }

    if employee_list.is_empty(){
        println!("There are no any employee in {:?}", department);
    }
    else{
        for employee in employee_list{
            dbg!(&employee);
            println!("\n");
        }
    }
    
}

fn show_data_all(employee_vect: &Vec<Employee>){

    println!("      All employees:\n");

    let mut employee_list = Vec::new();
    for worker in employee_vect{   
        employee_list.push(worker);
    }

    if employee_list.is_empty(){
        println!("There are no any employees");
    }
    else{
        for el in employee_list{
            dbg!(&el);
            println!("\n");
        }
    }
    
}

fn get_employee_name(user_data: & mut String) -> Option<String>{
    
    let res = match user_data.find("add "){
        Some(first_name_index) => {

            match user_data.find(" to "){
                Some(last_name_index) =>{
                    if last_name_index > first_name_index{
                        println!("last_name_index: {0}   first_name_index: {1}",last_name_index,first_name_index);
                        let employee_name: String = user_data.drain(first_name_index+3..last_name_index).collect();
                        Option::Some(employee_name.trim().to_string())
                    }
                   else{
                    Option::None
                   }
                }
                None => Option::None
            }
        },
        None => Option::None
    };

dbg!(&res);
res
}

fn get_employee_department(user_data: & mut String) -> Option<String>{
    match user_data.find(" to "){
        Some(first_dep_index) =>{
            let department_name: String = user_data.drain(first_dep_index+3..).collect();
            Option::Some(department_name.trim().to_string())
        },
        None => Option::None
    }
}

fn find_the_department(employee_department: &String) -> Option<Department>{
    match employee_department.as_str(){
        "developer" => Option::Some(Department::developer),
        "admin" => Option::Some(Department::admin),
        "engineering" => Option::Some(Department::engineering),
        "accountant" => Option::Some(Department::accountant),
        _=> Option::None
    }
}



// ---commands in line---


fn rebase_empl(employee_vect: &mut Vec<Employee>){
    println!("\n      Rebasing the Employee\n");
    let message = format!("Enter {0}add <employee_name> to <department>{0}", QUOT);

let data_to_rebase = 'enter_data_rebase: loop{
    match enter_string(&message){
        Option::Some(mut user_data) => {

            let employee_name = match get_employee_name(& mut user_data){
                Option::Some(employee_name) => {
                    if employee_name.is_empty(){
                        {println!("Enter the employee name!"); continue 'enter_data_rebase}
                    }
                    else{
                        match find_the_name(employee_vect, &employee_name){
                            Option::Some(employee_name) => employee_name,
                            Option::None => {println!("There are no any emplyees like {0}{1}{0}! Try again", QUOT, employee_name); continue 'enter_data_rebase}
                        }
                    }
                },
                Option::None => {println!("The command should looks like {0}add <employee_name> to <department>{0}!", QUOT); continue 'enter_data_rebase}
            };
        
            let employee_department = match get_employee_department(& mut user_data){
                Option::Some(employee_department) => {
                    if employee_department.is_empty(){
                        {println!("Enter the employee department!"); continue 'enter_data_rebase}
                    }
                    else{
                        match find_the_department(&employee_department){
                            Option::Some(employee_department) => employee_department,
                            Option::None => {println!("There are no such departments like {0}{1}{0}! Try again", QUOT, employee_department); continue 'enter_data_rebase}
                        }
                    }
                },
                Option::None => {println!("The command should looks like {0}add <employee_name> to <department>{0}!", QUOT); continue 'enter_data_rebase}
            };

            let data_to_rebase = Data_to_rebase{
                name: employee_name,
                department: employee_department
            };
            dbg!(&data_to_rebase);
            break 'enter_data_rebase data_to_rebase
        },
        Option::None => {println!("Enter something!"); continue 'enter_data_rebase} 
    }
};

    let mut iterator = employee_vect.iter_mut();

    'find_empl_to_rebase: loop{
        match iterator.next(){
            Some(worker) =>{
                if worker.name == data_to_rebase.name{     
                    println!("Success! The emplyee {0}{1}{0} was rebased from {0}{3:#?}{0} to {0}{2:#?}{0}.", QUOT, data_to_rebase.name, data_to_rebase.department ,worker.department);
                    worker.department = data_to_rebase.department;                   
                    break 'find_empl_to_rebase
                }
            },
            None => break 'find_empl_to_rebase
        }
    }
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