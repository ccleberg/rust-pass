use clap::{Arg, App};
use cli_table::{print_stdout, Cell, Style, Table};
use rand::{thread_rng, Rng};
use rusqlite::{Connection, Result};
use std::{fs, str};
use uuid::Uuid;

pub const SQLITE_DB: &str = "vault.sqlite";
pub const KEY_FILE: &str = "vault.key";
pub const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const NUMBERS: &str = "0123456789";
pub const SYMBOLS: &str = ")(*&^%$#@!~";
static DEFAULT_WORDLIST: &'static str = include_str!("wordlist.txt");

#[derive(Debug)]
struct Account {
    uuid: String,
    title: String,
    username: String,
    password: String,
    url: String,
}

// Read user input as a string
fn read_string() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input = input.trim().to_string();
    cleaned_input
}

// Read user input as a 32-bit unsigned integer
fn read_integer() -> u32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    let cleaned_input: u32 = input.trim().parse().expect("Error");
    cleaned_input
}

// Generate a random password string
fn generate_password(n: u32) -> String {
    // Get a random list of characters
    let mut charset = String::from(UPPERCASE);
    charset.push_str(LOWERCASE);
    charset.push_str(SYMBOLS);
    charset.push_str(NUMBERS);
    let char_vec: Vec<char> = charset.chars().collect();

    // Map random characters to a password
    let mut rng = rand::thread_rng();
    let password: String = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..char_vec.len());
            char_vec[idx] as char
        })
        .collect();
    password
}

// Generate a random passphrase string
fn generate_passphrase(n: u32, passphrase_symbol: String) -> String {
    // Load the words from file
    let words: Vec<&str> = DEFAULT_WORDLIST.lines()
        .collect();

    // Get random words
    let len = words.len();
    let mut rng = thread_rng();
    let password_words: Vec<&str> = (0..n)
        .map(|_| words[(rng.gen::<usize>() % len) - 1])
        .collect();

    // Join passphrase together with a symbol
    let passphrase = password_words.join(&*passphrase_symbol);
    passphrase
}

// Create the database table, if it doesn't exist
fn create_db() -> Result<()> {
    let conn = Connection::open(SQLITE_DB)?;
    conn.execute(
        "create table if not exists accounts (
             uuid text,
             application text,
             username text,
             password text,
             url text
         )",
        [],
    )?;
    Ok(())
}

// Insert data into the database
fn insert_account(uuid: String, application: String, username: String, password: String, url: String) -> Result<()> {
    let conn = Connection::open(SQLITE_DB)?;
    conn.execute(
        "INSERT INTO accounts (uuid, application, username, password, url) values (?1, ?2, ?3, ?4, ?5)",
        [uuid, application, username, password, url],
    )?;
    Ok(())
}

// Delete data from the database
fn update_account(uuid: String, field_string: String, new_value: String) -> Result<()> {
    let mut field: usize = usize::MAX;
    if (field_string == "title") | (field_string == "Title") {
        field = 0;
    } else if (field_string == "username") | (field_string == "Username") {
        field = 1;
    } else if (field_string == "password") | (field_string == "Password") {
        field = 2;
    } else if (field_string == "url") | (field_string == "URL") {
        field = 3;
    } else {
        eprintln!("Error: Provided field to edit does not match a field in the database.");
    }
    println!("Field: {}: {}", field_string, field);
    println!("New Value/UUID: {}: {}", new_value, uuid);
    let queries = vec![
        "UPDATE accounts SET application = ?1 WHERE uuid = ?2",
        "UPDATE accounts SET username = ?1 WHERE uuid = ?2",
        "UPDATE accounts SET password = ?1 WHERE uuid = ?2",
        "UPDATE accounts SET url = ?1 WHERE uuid = ?2",
    ];
    println!("Query: {}", queries[field]);
    let conn = Connection::open(SQLITE_DB)?;
    conn.execute(
        queries[field],
        [new_value, uuid],
    )?;
    Ok(())
}

// Delete data from the database
fn delete_account(uuid: String) -> Result<()> {
    let conn = Connection::open(SQLITE_DB)?;
    conn.execute(
        "DELETE FROM accounts WHERE uuid = ?1",
        [uuid],
    )?;
    Ok(())
}

// Read all records from the database and print
fn read_db() -> Result<()> {
    // Connect to the database and select all accounts
    let conn = Connection::open(SQLITE_DB)?;
    let mut stmt = conn.prepare(
        "SELECT * from accounts",
    )?;

    // Map each account returned from SQLite to an Account struct
    let accounts = stmt.query_map([], |row| {
        Ok(Account {
            uuid: row.get(0)?,
            title: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
            url: row.get(4)?,
        })
    })?;

    // Loop through saved accounts and collect them in a vec
    let mut tmp_table = vec![];
    for account in accounts {
        let tmp_account = account.unwrap();
        tmp_table.push(
            vec![
                decrypt(tmp_account.uuid).cell(),
                decrypt(tmp_account.title).cell(),
                decrypt(tmp_account.username).cell(),
                decrypt(tmp_account.password).cell(),
                decrypt(tmp_account.url).cell(),
            ]
        );
    }

    // Create a new, non-mutable vec to display
    let table = tmp_table
        .table()
        .title(vec![
            "UUID".cell().bold(true),
            "Title".cell().bold(true),
            "Username".cell().bold(true),
            "Password".cell().bold(true),
            "URL".cell().bold(true),
        ])
        .bold(true);

    assert!(print_stdout(table).is_ok());
    Ok(())
}

// Generate a new account
fn new() {
    // Generate UUID
    let uuid = Uuid::new_v4();
    println!("UUID: {}", uuid);

    // Gather input
    println!("Enter a title for this account:");
    let title = read_string();

    println!("Enter your username:");
    let username = read_string();

    println!("(Optional) Enter a URL for this account:");
    let url = read_string();

    let password: String = loop {
        println!("Do you want an XKCD-style passphrase [1] or a random password [2]? (1/2)");
        let password_choice = read_integer();
        if password_choice == 1 {
            let passphrase_words = loop {
                println!("Please enter number of words to include (min. 4):");
                let passphrase_words = read_integer();
                if passphrase_words >= 3 {
                    break passphrase_words;
                }
                println!("Invalid length. Please enter a number >= 3.");
            };
            println!("Please enter your desired separator symbol (_, -, ~, etc.:");
            let passphrase_symbol = read_string();
            let password = generate_passphrase(passphrase_words, passphrase_symbol);
            break password;
        } else if password_choice == 2 {
            let password_length = loop {
                println!("Please enter desired password length (min. 8):");
                let password_length = read_integer();
                if password_length >= 8 {
                    break password_length;
                }
                println!("Invalid length. Please enter a number >= 8.");
            };
            let password = generate_password(password_length);
            break password;
        }
        println!("Invalid response. Please respond with 1 or 2.");
    };

    // Generate an Account struct
    let account = Account {
        uuid: encrypt(uuid.to_string()),
        title: encrypt(title),
        username: encrypt(username),
        password: encrypt(password),
        url: encrypt(url),
    };

    // Create the database, if necessary, and insert data
    create_db();
    insert_account(account.uuid, account.title, account.username, account.password, account.url);
    println!("Account saved to the vault. Use `rpass --list` to see all saved accounts.");
}

// List all saved accounts
fn list() -> Result<()> {
    read_db();
    Ok(())
}

// TODO: Edit a saved account
// WARNING: This process does not currently work as expected; /
// I think the encrypted UUID differs from the encrypted UUID in the database
fn edit(uuid: String, field_name: String, new_value: String) {
    update_account(encrypt(uuid), field_name, encrypt(new_value));
}

// TODO: Delete a saved account
// WARNING: This process does not currently work as expected; /
// I think the encrypted UUID differs from the encrypted UUID in the database
fn delete(uuid: String) {
    delete_account(uuid);
}

// TODO: Delete all saved accounts and delete the vault file
fn purge() {
    println!();
}

// Encrypt plaintext using a generated key file
fn encrypt(plaintext: String) -> String {
    let key_exists: bool = std::path::Path::new(KEY_FILE).exists();
    let mut key = String::from("");
    if key_exists {
        key = fs::read_to_string(KEY_FILE).expect("Unable to read saved key file.");
    } else {
        key = fernet::Fernet::generate_key();
        fs::write(KEY_FILE, &key).expect("Unable to save key to file.");
        println!("Key file has been written to: {}. DO NOT DELETE OR MODIFY THIS FILE.", KEY_FILE);
    }
    let fernet = fernet::Fernet::new(&key).unwrap();
    let ciphertext = fernet.encrypt(plaintext.as_ref());
    ciphertext
}

// Decrypt ciphertext using a saved key file
fn decrypt(ciphertext: String) -> String {
    let key = fs::read_to_string(KEY_FILE).expect("Unable to read saved key file.");
    let fernet = fernet::Fernet::new(&key).unwrap();
    let decrypted_plaintext = fernet.decrypt(&ciphertext).expect("Error decrypting data - the key file may have been modified or deleted.");
    let plaintext = String::from_utf8(decrypted_plaintext).unwrap();
    plaintext
}

// Interpret user commands
fn main() {
    let matches = App::new("rpass")
        .version("1.1")
        .author("Christian Cleberg <hello@cmc.pub>")
        .about("A safe and convenient command-line password vault.")
        .arg(Arg::with_name("new")
            .short("n")
            .long("new")
            .help("Create a new account")
            .takes_value(false))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("List all saved accounts")
            .takes_value(false))
        .arg(Arg::with_name("edit")
            .short("e")
            .long("edit")
            .help("Edit a saved account")
            .value_names(&["uuid", "field_name", "new_value"])
            .takes_value(true))
        .arg(Arg::with_name("delete")
            .short("d")
            .long("delete")
            .help("Delete a saved account")
            .value_name("uuid")
            .takes_value(true))
        .arg(Arg::with_name("purge")
            .short("p")
            .long("purge")
            .help("Purge all saved accounts")
            .takes_value(false))
        .get_matches();

    if matches.is_present("new") {
        new();
    } else if matches.is_present("list") {
        list();
    } else if matches.is_present("edit") {
        let values: Vec<_> = matches.values_of("edit").unwrap().collect();
        edit(
            String::from(values[0]),
            String::from(values[1]),
            String::from(values[2]),
        );
    } else if matches.is_present("delete") {
        let values: Vec<_> = matches.values_of("delete").unwrap().collect();
        delete(String::from(values[0]));
    } else if matches.is_present("purge") {
        purge();
    }
}
