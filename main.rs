use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};

fn main() {
    start();
}

fn start() {
    println!("/////////////////////////////////");
    println!("// Welcome to Password manager //");
    println!("/////////////////////////////////");
    let _file = File::create("C:\\Users\\Public\\info.txt")
        .expect("Error encountered while creating file!");
    let _file = File::create("C:\\Users\\Public\\list.bag")
        .expect("Error encountered while creating file!");
    start_check();
}

fn start_check() {
    let mut response = 'n';
    let check_r =
        BufReader::new(File::open("C:\\Users\\Public\\info.txt").expect("File couldnt be opened!"));
    for line in check_r.lines() {
        if line.unwrap() != "" {
            response = 'y';
            login_acc();
        }
    }
    if response == 'n' {
        println!(
            "It seems like this is your first time using our app, please create an account first."
        );
        register_acc();
    }
}

fn register_acc() {
    let mut email = String::new();
    let mut password = String::new();
    print!("Enter your email: ");
    let _err = std::io::stdout().flush().unwrap();
    let _err = std::io::stdin().read_line(&mut email);
    if !(email.contains('@') && email.contains('.') && email.len() > 5) {
        println!("Email is invalid....");
        register_acc();
    }
    print!("Great, now enter your password: ");
    let _err = std::io::stdout().flush().unwrap();
    let _err = std::io::stdin().read_line(&mut password);
    let app_credentials = File::options()
        .append(true)
        .open("C:\\Users\\Public\\info.txt");
    let _err = write!(app_credentials.as_ref().unwrap(), "Email: {}", email);
    let _err = write!(app_credentials.as_ref().unwrap(), "Password: {}", password);
    println!("Great, now login.");
    login_acc();
}

fn login_acc() {
    let mut email_done: bool = false;
    let mut password_done: bool = false;
    let mut email = String::new();
    let mut password = String::new();
    print!("Enter your email: ");
    let _err = std::io::stdout().flush().unwrap();
    let _err = std::io::stdin().read_line(&mut email);
    email = email.trim().to_string();
    let app_credentials =
        BufReader::new(File::open("C:\\Users\\Public\\info.txt").expect("File couldnt be opened!"));
    for line in app_credentials.lines() {
        if line.as_ref().unwrap().contains("Email: ")
            && line.as_ref().unwrap()[7..line.as_ref().unwrap().len()].to_string() == email
        {
            email_done = true;
            break;
        }
    }
    if email_done == false {
        println!("Wrong email");
        login_acc();
    }
    print!("Enter your password: ");
    let _err = std::io::stdout().flush();
    let _err = std::io::stdin().read_line(&mut password);
    password = password.trim().to_string();
    let app_credentials =
        BufReader::new(File::open("C:\\Users\\Public\\info.txt").expect("File couldnt be opened!"));
    for line in app_credentials.lines() {
        if line.as_ref().unwrap().contains("Password: ")
            && line.as_ref().unwrap()[10..line.as_ref().unwrap().len()].to_string() == password
        {
            password_done = true;
            break;
        }
    }
    if password_done == false {
        println!("Wrong password");
        login_acc();
    }
    main_app();
}

fn main_app() {
    let mut response = String::new();
    let mut domain = String::new();
    let mut email = String::new();
    let mut password = String::new();
    print!("Want to add an account or to see the added ones?(add/show/quit): ");
    let _err = std::io::stdout().flush();
    let _err = std::io::stdin().read_line(&mut response);
    response = response.trim().to_string();
    if response == "add" {
        print!("Domain: ");
        let _err = std::io::stdout().flush();
        let _err = std::io::stdin().read_line(&mut domain);
        if domain.is_empty() == true {
            println!("Domain cannot be empty.");
            main_app();
        }
        print!("Email: ");
        let _err = std::io::stdout().flush();
        let _err = std::io::stdin().read_line(&mut email);
        if !(email.contains('@') && email.contains('.') && email.len() > 5
            || email.is_empty() == true)
        {
            println!("Incorrect email format!");
            main_app();
        }
        print!("Password: ");
        let _err = std::io::stdout().flush();
        let _err = std::io::stdin().read_line(&mut password);
        if password.is_empty() == true {
            println!("Password cannot be empty");
            main_app();
        }
        let adder = File::options()
            .append(true)
            .open("C:\\Users\\Public\\list.bag");
        if fs::metadata("C:\\Users\\Public\\list.bag").unwrap().len() == 0 {
            if email.len() + 10 >= password.len() + 13 {
                for _i in 0..email.len() + 10 {
                    let _err = write!(adder.as_ref().unwrap(), "-");
                }
            } else {
                for _i in 0..password.len() + 13 {
                    let _err = write!(adder.as_ref().unwrap(), "-");
                }
            }
            let _err = writeln!(adder.as_ref().unwrap());
        }
        let _err = write!(
            adder.as_ref().unwrap(),
            "Domain: {} ->Email: {} ->Password: {}",
            domain,
            email,
            password,
        );
        if email.len() + 10 >= password.len() + 13 {
            for _i in 0..email.len() + 10 {
                let _err = write!(adder.as_ref().unwrap(), "-");
            }
        } else {
            for _i in 0..password.len() + 13 {
                let _err = write!(adder.as_ref().unwrap(), "-");
            }
        }
        let _err = writeln!(adder.as_ref().unwrap());
        println!();
        main_app();
    } else if response == "show" {
        let reader =
            BufReader::new(File::open("C:\\Users\\Public\\list.bag").expect("Couldnt open file"));
        for line in reader.lines() {
            println!("{}", line.unwrap());
        }
        main_app();
    } else if response == "quit" {
        std::process::exit(0);
    } else {
        println!("Invalid response.");
        main_app();
    }
}
