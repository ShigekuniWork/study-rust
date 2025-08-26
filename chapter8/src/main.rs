use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum DepartmentKind {
    Development,
    Sales,
}

#[derive(Debug)]
pub struct Department<'a> {
    name: DepartmentKind,
    pub members: Vec<&'a str>,
}

impl<'a> Department<'a> {
    fn new(name: DepartmentKind) -> Self {
        Self { name, members: Vec::new() }
    }

    fn add_member(&mut self, member: &'a str) {
        self.members.push(member);
        println!("success {:?}", self.name);
    }
}

#[derive(Debug)]
struct Company<'a> {
    departments: HashMap<DepartmentKind, Department<'a>>,
}

impl<'a> Company<'a> {
    fn new() -> Self {
        Self { departments: HashMap::new() }
    }

    fn ensure_department(&mut self, kind: DepartmentKind) -> &mut Department<'a> {
        self.departments.entry(kind).or_insert_with(|| Department::new(kind))
    }

    fn add_department_member(&mut self, department: DepartmentKind, member: &'a str) {
        self.ensure_department(department).add_member(member);
    }

    fn get_members(&self, department: DepartmentKind) -> Option<&[&'a str]> {
        self.departments.get(&department).map(|d| d.members.as_slice())
    }
    
    fn get_sorted_members(&self, department: DepartmentKind) -> Option<Vec<&'a str>> {
        self.departments.get(&department).map(|d| {
            let mut v = d.members.clone();
            v.sort();
            v
        })
    }
}

fn main() {
    let alice = String::from("Alice");
    let bob = String::from("Bob");
    let carol = String::from("Carol");

    let mut company = Company::new();
    company.add_department_member(DepartmentKind::Development, &alice);
    company.add_department_member(DepartmentKind::Development, &bob);
    company.add_department_member(DepartmentKind::Sales, &carol);

    if let Some(members) = company.get_members(DepartmentKind::Development) {
        println!("Get Development {:?} : {:?}",DepartmentKind::Development, members);
    }
    if let Some(members) = company.get_members(DepartmentKind::Sales) {
        println!("Sales: {:?}", members);
    }
    if let Some(members) = company.get_sorted_members(DepartmentKind::Development) {
        println!("Sorted Development: {:?}", members);
    }
}