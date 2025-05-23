mod db;
mod encryption;

use std::io::{self, Write};
use db::{initialize_db, register_user, authenticate_user, store_message, fetch_unread_messages, delete_message};
use encryption::{encrypt_message, decrypt_message};
use std::thread;
use std::time::Duration;

// ANSI escape codes for CLI UI styling
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const CLEAR: &str = "\x1b[2J\x1b[H"; // Clear screen

fn main() {
    initialize_db();
    loop {
        clear_screen();
        print_title();

        println!("\n{}========================={}", GREEN, RESET);
        println!("{}|      MILITARY COMS     |{}", BOLD, RESET);
        println!("{}========================={}", GREEN, RESET);
        println!("{}1. Login{}", GREEN, RESET);
        println!("{}2. Signup{}", GREEN, RESET);
        println!("{}3. Exit{}", GREEN, RESET);
        println!("{}========================={}", GREEN, RESET);

        let choice = get_input("Enter your numeric choice: ");

        match choice.as_str() {
            "1" => login_user(),
            "2" => {
                signup_user();
                continue; // Re-display the main menu after signup
            }
            "3" => {
                println!("{}Exiting...{}", GREEN, RESET);
                break;
            }
            _ => println!("{}Invalid choice, try again.{}", GREEN, RESET),
        }
    }
}

fn clear_screen() {
    print!("{}", CLEAR);
    io::stdout().flush().unwrap();
}

fn print_title() {
    let title = r#"
  ░▒▓██████▓▒░  ░▒▓██████▓▒░  ░▒▓██████████████▓▒░  ░▒▓██████████████▓▒░  ░▒▓███████▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        
░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        
░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  
░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░ 
 ░▒▓██████▓▒░   ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░  
    "#;
    println!("{}{}", GREEN, title);
    println!("{}SECURED MILITARY TELECOMMUNICATION CHANNEL{}\n", BOLD, RESET);
}

fn login_user() {
    let username = get_input("Enter username: ");
    let password = get_input("Enter password: ");

    if authenticate_user(&username, &password) {
        clear_screen();
        print_cryptix_title();
        user_menu(username);
    } else {
        println!("{}Invalid username or password.{}", GREEN, RESET);
    }
}

fn print_cryptix_title() {
    let cryptix_title = r#"
 ░▒▓██████▓▒░  ░▒▓███████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░  ░▒▓████████▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░    ░▒▓█▓▒░     ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░    ░▒▓█▓▒░     ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░        ░▒▓███████▓▒░   ░▒▓██████▓▒░  ░▒▓███████▓▒░     ░▒▓█▓▒░     ░▒▓█▓▒░  ░▒▓██████▓▒░  
░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░    ░▒▓█▓▒░     ░▒▓█▓▒░           ░▒▓█▓▒░     ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░    ░▒▓█▓▒░     ░▒▓█▓▒░           ░▒▓█▓▒░     ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ 
 ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░    ░▒▓█▓▒░     ░▒▓█▓▒░           ░▒▓█▓▒░     ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░
    "#;
    println!("{}{}", GREEN, cryptix_title);
    println!("{}WELCOME TO CRYPTIX SECURE CHAT SYSTEM{}\n", BOLD, RESET);
}

fn signup_user() {
    let username = get_input("Choose a username: ");
    let password = get_input("Choose a password: ");

    if register_user(&username, &password) {
        println!("{}Signup successful! Please login.{}", GREEN, RESET);
        println!("{}Returning to main menu...{}", GREEN, RESET);
        thread::sleep(Duration::from_secs(2)); // 2-second delay before returning
    } else {
        println!("{}Username already exists, please try another.{}", GREEN, RESET);
    }
}

fn user_menu(username: String) {
    loop {
        println!("\n{}========================={}", GREEN, RESET);
        println!("{}|        MAIN MENU      |{}", BOLD, RESET);
        println!("{}========================={}", GREEN, RESET);
        println!("{}1. Send Message{}", GREEN, RESET);
        println!("{}2. View Messages{}", GREEN, RESET);
        println!("{}3. Logout{}", GREEN, RESET);
        println!("{}========================={}", GREEN, RESET);

        let choice = get_input("Enter your numeric choice: ");

        match choice.as_str() {
            "1" => send_message(&username),
            "2" => view_messages(&username),
            "3" => {
                println!("{}Logging out...{}", GREEN, RESET);
                break;
            }
            _ => println!("{}Invalid option, try again.{}", GREEN, RESET),
        }
    }
}

fn send_message(sender: &str) {
    let receiver = get_input("Enter receiver's username: ");
    let message = get_input("Enter your message: ");
    let secret = get_input("Enter a secret passphrase: ");

    if receiver.is_empty() || message.is_empty() || secret.is_empty() {
        println!("{}All fields are required.{}", GREEN, RESET);
        return;
    }

    match encrypt_message(&secret, &message) {
        Ok((iv, encrypted_text)) => {
            store_message(sender, &receiver, &iv, &encrypted_text);
            println!("{}Message sent!{}", GREEN, RESET);
            io::stdout().flush().unwrap(); // Ensure message is printed before delay
            thread::sleep(Duration::from_secs(2)); // 2-second delay before clearing
        }
        Err(e) => println!("{}", e),
    }
}

fn view_messages(username: &str) {
    let messages = fetch_unread_messages(username);
    if messages.is_empty() {
        println!("{}No unread messages.{}", GREEN, RESET);
        return;
    }

    for (id, sender, iv, encrypted_text) in messages {
        println!("{}Message from {}:{}", GREEN, sender, RESET);
        let secret = get_input("Enter secret passphrase: ");

        match decrypt_message(&secret, &iv, &encrypted_text) {
            Ok(plaintext) => {
                println!("{}Decrypted Message:{} {}", GREEN, RESET, plaintext);
                delete_message(id);
                thread::sleep(Duration::from_secs(2)); // 2-second delay before clearing
            }
            Err(e) => println!("{}", e),
        }
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}{}", GREEN, prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
