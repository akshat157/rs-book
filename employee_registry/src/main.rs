use std::collections::HashMap;
use std::io;
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
fn create_department(dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    if reg.contains_key(dept) {
        println!("Department already exists: '{dept}'");
    } else {
        reg.insert(dept.to_string(), Vec::new());
        println!("Department created: {dept}");
    }
}

fn delete_department(dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    match reg.remove(dept) {
        Some(_) => println!("Department deleted: {dept}"),
        None => println!("No department found by name {dept}"),
    }
}

fn add_employee(emp: &str, dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    match reg.get_mut(dept) {
        Some(dept_emps) => {
            dept_emps.push(emp.to_string());
            dept_emps.sort();
            println!("Employee added: {emp} to {dept}.");
        }
        None => {
            println!("No department found by name {dept}");
        }
    }
}

fn remove_employee(emp: &str, dept: &str, reg: &mut HashMap<String, Vec<String>>) {
    match reg.get_mut(dept) {
        Some(dept_emps) => {
            let mut i: Option<usize> = None;
            for (idx, val) in dept_emps.iter().enumerate() {
                if val == emp {
                    i = Some(idx);
                    break;
                }
            }
            if let Some(i) = i {
                dept_emps.remove(i);
            }
            println!("Employee removed: {emp} from {dept}.");
        }
        None => {
            println!("No department found by name {dept}");
        }
    }
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
    // TODO: Optimize this method. Currently causes unnecessary repeated lookups on the HashMap.
    // Can be replaced with raw implementation of the display method.
    for (dept, emps) in reg {
        println!("Showing employees for '{dept}'");
        for emp in emps {
            println!("{emp}");
        }
    }
}

fn print_invalid_prompt() {
    println!("Invalid prompt!");
}

fn print_instructions() {
    println!("Use the following options to perform the desired operations.");
    println!("1. create <department>\t: Create a new department named <department>.");
    println!(
        "2. delete <department>\t: Delete the department named <department>.\n\t\t\t  Deletes the list of employees in that department as well!"
    );
    println!(
        "3. show <department>\t: show the list of employees in department <department>, sorted alphabetically."
    );
    println!(
        "4. showall\t: show the all departments in the company with all employees, sorted alphabetically."
    );
    println!(
        "5. add <employee> to <department>\t: add employee <employee> to department <department>."
    );
    println!(
        "6. remove <employee> from <department>\t: remove employee <employee> from department <department>."
    );
    println!("7. help\t: print the help text.");
    println!("8. quit\t: exit the program.");
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

        match pieces.len() {
            1 => {
                if pieces[0] == "help" {
                    print_instructions();
                } else if pieces[0] == "quit" {
                    break;
                } else if pieces[0] == "showall" {
                    show_all_departments(&registry);
                } else {
                    print_invalid_prompt();
                }
            }
            2 => {
                if pieces[0] == "create" {
                    create_department(pieces[1], &mut registry);
                } else if pieces[0] == "delete" {
                    delete_department(pieces[1], &mut registry);
                } else if pieces[0] == "show" {
                    show_department(pieces[1], &registry)
                } else {
                    print_invalid_prompt();
                }
            }
            4 => {
                if pieces[0] == "add" && pieces[2] == "to" {
                    add_employee(pieces[1], pieces[3], &mut registry);
                } else if pieces[0] == "remove" && pieces[2] == "from" {
                    remove_employee(pieces[1], pieces[3], &mut registry);
                } else {
                    print_invalid_prompt();
                }
            }
            _ => print_invalid_prompt(),
        }
    }
}
