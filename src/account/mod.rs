use fake::{Dummy, Fake, Faker};
use rand::Rng;
use std::usize;

// ex1
#[derive(Debug, Dummy, Clone)]
struct Account {
    #[dummy(faker = "1..=1000")]
    id: usize,
    username: String,
    display_name: String,
}

impl Account {
    fn new(id: usize, username: &str, display_name: &str) -> Account {
        Account {
            id: usize::from(id),
            username: String::from(username),
            display_name: String::from(display_name),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AccountError {
    Unknown,
}

pub fn create(id: usize, username: &str, display_name: &str) -> Account {
    let mut rng = rand::thread_rng();
    let new_id: usize = rng.gen_range(1001..2000);
    // ex2
    Account::new(id, username, display_name);
}

pub fn list() -> Result<Vec<Account>, AccountError> {
    // ex3
    // ???.map(|_| Faker.fake()).collect::<Vec<Account>>();
    Ok(vec![])
}

pub fn get_by_username(username: &str) -> Result<Account, AccountError> {
    println!("searching a {} user...", username);
    // ex4
}
