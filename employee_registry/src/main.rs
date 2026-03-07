use std::collections::HashMap;
use std::io;
use std::process::exit;
// struct Company {
//    departments: Vec<Department>,
//    name: String,
//    employees: Vec<Employee>,
// }

// struct Department {
//    name: String,
// }

// struct Employee {
//     name: String,
// }
fn exit_program() {
    println!("Exit.");
    exit(0);
}

fn create_department(dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    println!("Create {dept}");
    reg.insert(dept.to_string(), Vec::new());
    // TODO: Error handling
    println!("{reg:?}");
}

fn delete_department(dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    println!("Delete {dept}");
    reg.remove(&dept.to_string());
    // TODO: Error handling
    println!("{reg:?}");
}

fn add_employee(emp: &str, dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    println!("Add {emp} to {dept}.");
    match reg.get_mut(dept) {
        Some(dept_emps) => {
            dept_emps.push(emp.to_string());
        }
        None => {
            println!("No department found by name {dept}");
        }
    }
    println!("{reg:?}");
}

fn remove_employee(emp: &str, dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    println!("Remove {emp} from {dept}.");
    match reg.get_mut(dept) {
        Some(dept_emps) => {
            let mut i: Option<usize> = None;
            for (idx, val) in dept_emps.iter_mut().enumerate() {
                if val == emp {
                    i = Some(idx);
                    break;
                }
            }
            if let Some(i) = i {
                dept_emps.remove(i);
            }
        }
        None => {
            println!("No department found by name {dept}");
        }
    }
    println!("{reg:?}");
}

fn show_department(dept: &str, reg: &HashMap<String, Vec<String>>) {
    match reg.get(dept) {
        Some(dept_emps) => {
            println!("Showing employees for '{dept}'");
            for emp in dept_emps {
                println!("{emp}");
            }
        }
        None => println!("No department found by name {dept}"),
    }
}

fn show_all_departments(reg: &HashMap<String, Vec<String>>) {
    for dept in reg.keys() {
        show_department(dept, reg);
    }
}

fn print_invalid_prompt() {
    println!("Invalid prompt!");
}

fn print_instructions() {
    println!("Use the following options to perform the desired operations.");
    println!("create <department>\t: Create a new department named <department>.");
    println!(
        "delete <department>\t: Delete the department named <department>.\n\t\t\tDeletes the list of employees in that department as well!"
    );
    println!(
        "show <department>\t: show the list of employees in department <department>, sorted alphabetically."
    );
    println!(
        "showall\t: show the all departments in the company with all employees, sorted alphabetically."
    );
    println!(
        "add <employee> to <department>\t: Add employee <employee> to department <department>."
    );
    println!(
        "remove <employee> from <department>\t: Add employee <employee> to department <department>."
    );
    println!("help\t: print the help text.");
    println!("quit\t: quit the program.");
}

fn main() {
    // Create a hashmap to store departments as keys and each will be a vector of strings to store
    // employee names.
    println!("+-------------------------------------+");
    println!("|-----Akshat's Company Department-----|");
    println!("|----and Employee Registry Manager----|");
    println!("+-------------------------------------+");

    print_instructions();
    let mut registry: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut prompt = String::new();
        io::stdin()
            .read_line(&mut prompt)
            .expect("Failed to read line");

        let pieces: Vec<&str> = prompt.trim().split_ascii_whitespace().collect();

        if !(pieces.len() == 1 || pieces.len() == 2 || pieces.len() == 4) {
            print_invalid_prompt();
            continue;
        }

        if pieces.len() == 1 {
            if pieces[0] == "help" {
                print_instructions();
            } else if pieces[0] == "quit" {
                exit_program();
            } else if pieces[0] == "showall" {
                show_all_departments(&registry);
            } else {
                print_invalid_prompt();
            }
        } else if pieces.len() == 2 {
            if pieces[0] == "create" {
                create_department(pieces[1], &mut registry);
            } else if pieces[0] == "delete" {
                delete_department(pieces[1], &mut registry);
            } else if pieces[0] == "show" {
                show_department(pieces[1], &registry)
            } else {
                print_invalid_prompt();
            }
        } else if pieces.len() == 4 {
            if pieces[0] == "add" && pieces[2] == "to" {
                add_employee(pieces[1], pieces[3], &mut registry);
            } else if pieces[0] == "remove" && pieces[2] == "from" {
                remove_employee(pieces[1], pieces[3], &mut registry);
            } else {
                print_invalid_prompt();
            }
        }
    }
}
