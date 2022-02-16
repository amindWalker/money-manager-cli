use std::{collections::HashMap, io};

fn main() {
    use crate::menu::{add_bill, display_bills, edit_bill, remove_bill};
    let mut bills_list = Bills::new();

    loop {
        Menu::program_title();
        display_bills(&bills_list);
        Menu::main_menu();

        use Menu::{AddBill, EditBill, Exit, RemoveBill};
        let input = get_input().expect("\x1b[48;5;3;38;5;16m Icorrect option. \x1b[0m");
        match Menu::selection(input.as_str()) {
            Some(AddBill) => add_bill(&mut bills_list),
            Some(EditBill) => edit_bill(&mut bills_list),
            Some(RemoveBill) => remove_bill(&mut bills_list),
            Some(Exit) => {
                println!("Saving and closing...");
                break;
            }
            _ => println!("\n\x1b[48;5;11;38;5;16m invalid option \x1b[0m\n"),
        };
    }
}

#[derive(Debug)]
enum Menu {
    AddBill,
    EditBill,
    RemoveBill,
    Exit,
}

impl Menu {
    fn program_title() {
        println!(
            "\x1b[1m\x1b[48;5;15;38;5;16m{: ^60}\x1b[0m",
            "EXPENSE MANAGER"
        );
    }
    fn main_menu() {
        println!("\x1b[38;5;15m\nChoose one option from 0-3:\n\t1. Create new bill\n\t2. Edit bill\n\t3. Remove bill\n\t0. Exit\n");
    }
    fn selection(input: &str) -> Option<Menu> {
        use Menu::{AddBill, EditBill, Exit, RemoveBill};
        match input {
            "1" => Some(AddBill),
            "2" => Some(EditBill),
            "3" => Some(RemoveBill),
            "0" => Some(Exit),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Bill {
    name: String,
    value: f64,
}

pub struct Bills {
    bill: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self {
        Self {
            bill: HashMap::new(),
        }
    }
    fn add(&mut self, bill: Bill) {
        self.bill.insert(bill.name.to_string(), bill);
    }
    fn get_all(&self) -> Vec<&Bill> {
        self.bill.values().collect()
    }
    fn remove(&mut self, name: &String) {
        self.bill.remove(name);
    }
    fn edit(&mut self, name: &str, value: f64) {
        match self.bill.get_mut(name) {
            Some(key) => key.value = value,
            _ => println!("\x1b[48;5;3;38;5;15mData not found\x1b[0m"),
        };
    }
}

mod menu {
    use crate::{get_input, get_input_parse_float, Bill, Bills};

    pub fn add_bill(bills_list: &mut Bills) {
        print!("\x1B[2J"); // Clear console
        println!("\nYou chose option: 1\n\x1b[48;5;10;38;5;16m Create new bill \x1b[0m\n\nAdd new bill title (empty to cancel): ");
        let title = match get_input() {
            Ok(value) => match value {
                value if &value == "" => return,
                value => value.to_lowercase(),
            },
            _ => return,
        };

        println!("\nAdd new bill value (accepted number formats: 100, 100.0 or 100.250): ");
        let parsed_value = get_input_parse_float();
        let parsed_value = match parsed_value {
            Some(value) => value,
            _ => return,
        };

        let key_value_bill = Bill {
            name: title,
            value: parsed_value,
        };
        println!("\n\x1b[48;5;28;38;5;15m NEW BILL ADDED \x1b[0m\n");
        bills_list.add(key_value_bill);
    }
    pub fn display_bills(bills_list: &Bills) {
        if bills_list.bill.is_empty() {
            println!("\n{: ^60}\n", "EMPTY LIST");
        }
        let mut count = 0;
        for bill in bills_list.get_all() {
            if count % 2 == 0 {
                println!(
                    "\t\x1b[48;5;249;38;5;16m {: ^30} \x1b[48;5;28;38;5;15m {: ^12?} \x1b[0m",
                    bill.name.to_uppercase(),
                    bill.value
                );
            } else {
                println!(
                    "\t\x1b[48;5;251;38;5;16m {: ^30} \x1b[48;5;34;38;5;15m {: ^12?} \x1b[0m",
                    bill.name.to_uppercase(),
                    bill.value
                );
            }
            count += 1
        }
    }
    pub fn remove_bill(bills_list: &mut Bills) {
        print!("\x1B[2J");
        'main: loop {
            if bills_list.bill.is_empty() {
                println!("\n\x1b[48;5;3;38;5;16m NO BILLS TO REMOVE \x1b[0m\n");
                break 'main;
            }
            println!("\nYou chose option: 3\n\x1b[48;5;9;38;5;16m Remove bill \x1b[0m");
            println!("\nType the name of the bill to be removed (empty to cancel):\n");
            display_bills(&bills_list);

            match get_input().as_mut() {
                Ok(input) => match input {
                    input if input.is_empty() => break 'main,
                    input if bills_list.bill.contains_key(&input.to_lowercase()) => {
                        bills_list.remove(&input.to_lowercase());
                        println!(
                            "\n\x1b[48;5;28;38;5;15m {} REMOVED \x1b[0m\n",
                            input.to_uppercase()
                        );
                        break;
                    }
                    _ => println!(
                        "\n\x1b[48;5;3;38;5;16m NO BILLS FOUND CONTAINING: {}\x1b[0m\n",
                        input
                    ),
                },
                _ => break 'main,
            }
        }
    }
    pub fn edit_bill(bills_list: &mut Bills) {
        print!("\x1B[2J");
        'main: loop {
            if bills_list.bill.is_empty() {
                println!("\n\x1b[48;5;3;38;5;16m NO BILLS TO EDIT \x1b[0m\n");
                break 'main;
            }
            println!("\nYou chose option: 3\n\x1b[48;5;4;38;5;16m Edit bill \x1b[0m");
            println!("\nType the name of the bill to be edited (empty to cancel):\n");
            display_bills(&bills_list);

            match get_input().as_mut() {
                Ok(input) => match input {
                    input if input.is_empty() => break 'main,
                    input if bills_list.bill.contains_key(&input.to_lowercase()) => {
                        println!("Type the new value for {input}");
                        let value = match get_input_parse_float() {
                            Some(val) => val,
                            _ => return,
                        };
                        bills_list.edit(&input.to_lowercase(), value);
                        println!(
                            "\n\x1b[48;5;28;38;5;15m {} UPDATED \x1b[0m\n",
                            input.to_uppercase()
                        );
                        break 'main;
                    }
                    _ => println!(
                        "\n\x1b[48;5;3;38;5;16m NO BILLS FOUND CONTAINING: {}\x1b[0m\n",
                        input
                    ),
                },
                _ => break 'main,
            }
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn get_input_parse_float() -> Option<f64> {
    loop {
        let value = match get_input() {
            Ok(input) => {
                if &input == "" {
                    return None;
                }
                input
            }
            _ => return None,
        };
        let parsed_value = value.parse();
        match parsed_value {
            Ok(value) => return Some(value),
            _ => println!("\n\x1b[48;5;3;38;5;16m IVALID FORMAT. VALID NUMBER FORMATS: 100, 100.0 or 100.250 \x1b[0m\n"),
        }
    }
}
