use std::{collections::HashMap, io};

fn main() {
    let mut bills_list: HashMap<String, f64> = HashMap::new();

    loop {
        println!(
            "\x1b[1m\x1b[48;5;15;38;5;16m{: ^60}\x1b[0m",
            "EXPENSE MANAGER"
        );
        display_bills(&bills_list);

        println!(
            "\x1b[38;5;15m\nChoose one option from 0-3:\n\t1. Create new bill\n\t2. Edit bill\n\t3. Remove bill\n\t0. Exit\n"
        );
        match get_input() {
            Ok(index) => match index {
                item if item.as_str() == "1" => add_bill(&mut bills_list),
                item if item.as_str() == "2" => (),
                item if item.as_str() == "3" => remove_bill(&mut bills_list),
                item if item.as_str() == "0" => {
                    println!("Saving and closing...");
                    break;
                },
                _item => println!(
                    "\nYou chose option: {_item}\n\x1b[48;5;11;38;5;16m there is no option {_item} \x1b[0m\n"
                ),
            },
            _ => return,
        }
    }
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn add_bill(bills_list: &mut HashMap<String, f64>) {
    print!("\x1B[2J"); // Clear console
    println!(
        "\nYou chose option: 1\n\x1b[48;5;10;38;5;16m Create new bill \x1b[0m\n\nAdd new bill title (empty to cancel): "
    );
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
        Some(value) => {
            println!("\n\x1b[48;5;28;38;5;15m NEW BILL ADDED \x1b[0m\n");
            value
        }
        _ => {
            println!(
                "\n\x1b[48;5;3;38;5;16m ICORRECT FORMAT. ACCEPTED NUMBER FORMATS: 100, 100.0 or 100.250 \x1b[0m\n"
            );
            return;
        }
    };

    bills_list.insert(title, parsed_value);
}

fn get_input_parse_float() -> Option<f64> {
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
        Ok(value) => Some(value),
        _ => None,
    }
}

fn display_bills(bills_list: &HashMap<String, f64>) {
    if bills_list.is_empty() {
        println!("\n{: ^60}\n", "EMPTY LIST");
    }

    let mut count = 0;
    for (key, value) in bills_list.iter() {
        if count % 2 == 0 {
            println!(
                "\t\x1b[48;5;249;38;5;16m {: ^30} \x1b[48;5;28;38;5;15m {value: ^12} \x1b[0m",
                key.to_uppercase()
            );
        } else {
            println!(
                "\t\x1b[48;5;251;38;5;16m {: ^30} \x1b[48;5;34;38;5;15m {value: ^12} \x1b[0m",
                key.to_uppercase()
            );
        }
        count += 1
    }
}

fn remove_bill(bills_list: &mut HashMap<String, f64>) {
    'main: loop {
        print!("\x1B[2J");
        if bills_list.is_empty() {
            println!("\n\x1b[48;5;3;38;5;16m NO BILLS TO REMOVE \x1b[0m\n");
            break 'main;
        }
        println!("\nYou chose option: 3\n\x1b[48;5;9;38;5;16m Remove bill \x1b[0m");
        println!("\nType the name of the bill to be removed (empty to cancel):\n");
        display_bills(&bills_list);

        match get_input().as_mut() {
            Ok(input) => match input {
                input if input.is_empty() => break 'main,
                input if bills_list.contains_key(&input.to_lowercase()) => {
                    bills_list.remove_entry(&input.to_lowercase());
                    println!(
                        "\n\x1b[48;5;28;38;5;15m {} REMOVED \x1b[0m\n",
                        input.to_uppercase()
                    );
                    break;
                }
                _ => println!(
                    "\n\x1b[48;5;3;38;5;16m NO BILLS FOUND CONTAINING: {}\x1b[0m\n",
                    input.to_uppercase()
                ),
            },
            _ => break 'main,
        }
    }
}
