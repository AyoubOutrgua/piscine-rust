#[derive(Debug, PartialEq, Clone)]
pub enum Role {
    CEO,
    Manager,
    Worker,
}

impl From<&str> for Role {
    fn from(s: &str) -> Self {
        match s {
            "CEO" => Role::CEO,
            "Manager" => Role::Manager,
            _ => Role::Worker,
        }
    }
}

#[derive(Debug)]
pub struct WorkEnvironment {
    pub grade: Link,
}

pub type Link = Option<Box<Worker>>;

#[derive(Debug)]
pub struct Worker {
    pub role: Role,
    pub name: String,
    pub next: Link,
}

impl WorkEnvironment {
    pub fn new() -> Self {
        Self { grade: None }
    }

    pub fn add_worker(&mut self, name: &str, role: &str) {
        let worker = Worker {
            role: Role::from(role),
            name: name.to_string(),
            next: self.grade.take(),
        };
        self.grade = Some(Box::new(worker))
    }

    pub fn remove_worker(&mut self) -> Option<String> {
        if let Some(worker) = self.grade.take() {
            let next_worker = worker.next;
            self.grade = next_worker;
            Some(worker.name)
        }else {
            None
        }
    }

    pub fn last_worker(&self) -> Option<(String, Role)> {
        if let Some(worker) = self.grade.as_ref() {
            let role = match worker.role {
                Role::CEO => Role::CEO,
                Role::Manager => Role::Manager,
                Role::Worker => Role::Worker,
            };
            Some((worker.name.clone(), role))
        } else {
            None
        }
    }
}
