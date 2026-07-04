use std::collections::HashMap;

pub mod mall;
pub use crate::mall::{Mall, Floor, Store, Employee, Guard};

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut data = HashMap::new();
    for floor in mall.floors.clone() {
        let stores = floor.1.stores.clone();
        for (string, store) in stores {
            data.insert(string, store);
        }
    }
    data.into_iter().max_by_key(|(_,v)| v.square_meters).unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut highest_paid: Vec<(String, Employee)> = Vec::new();

    let mut max_salary: f64 = 0.0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in store.employees.iter() {
                if max_salary < employee.salary {
                    max_salary = employee.salary;
                    highest_paid.clear();
                    highest_paid.push((name.clone(), employee.clone()));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total_employee: usize = 0;
    for _ in mall.guards.clone() {
        total_employee += 1;
    }
    for floor in mall.floors.clone() {
        for store in floor.1.stores.clone() {
            for _ in store.1.employees {
                total_employee += 1;
            }
        }
    }
    total_employee
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut total_area: u64 = 0;
    for floor in mall.floors.clone() {
        total_area += floor.1.size_limit;
    }
    let req_guards = (total_area / 200) as usize;
    let total_guards = mall.guards.len();

    if total_guards < req_guards {
        let num_add = req_guards - total_guards;
        for (name, guard) in guards.into_iter().take(num_add) {
            mall.hire_guard(name, guard);
        }
    }
}

