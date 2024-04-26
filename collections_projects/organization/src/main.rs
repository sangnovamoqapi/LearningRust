use std::{collections::HashMap, io};

fn add_employee(map: &mut HashMap<String, Vec<String>>) {
    let mut employee_name = String::new();
    let mut department_name = String::new();
    println!("Enter Employee name:");
    io::stdin().read_line(&mut employee_name);
    println!("Enter Department name:");
    io::stdin().read_line(&mut department_name);

    // Remove trailing newline characters
    employee_name = employee_name.trim().to_string();
    department_name = department_name.trim().to_string();
    map.entry(department_name.clone()).or_insert(Vec::new());
    let vec = map.get_mut(&department_name).unwrap();
    vec.push(employee_name);
}

fn get_employees_by_department(employees: &HashMap<String, Vec<String>>) {
    let mut department = String::new();
    println!("Enter the department");
    io::stdin().read_line(&mut department);
    for (key, value) in employees {
        if *key == department.trim().to_string() {
            println!("The employees in {key} department are:");
            for (index, employee) in value.iter().enumerate() {
                let sr = index + 1;
                println!("{sr}. {employee}");
            }
        }
    }
}

fn get_all_employees(employees: &HashMap<String, Vec<String>>) {
    for (key, value) in employees {
        println!("The employees in {key} department are:");
        for (index, employee) in value.iter().enumerate() {
            let sr = index + 1;
            println!("{sr}. {employee}");
        }
        println!();
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!(
            "1 to add an employee 
2 to get employees by department 
3 to get all employees 
4 to exit"
        );
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let case: i32 = input.trim().parse().expect("Invalid input");
        match case {
            1 => add_employee(&mut employees),
            2 => get_employees_by_department(&employees),
            3 => get_all_employees(&employees),
            4 => break,
            other => println!("Invalid case!"),
        }
    }
}
