use std::collections::HashMap;

struct Student {
    id: u32,
    name: String
}

struct Subject {
    name: String,
    classroom: String,
}

struct SchoolClass {
    id: u8,
    serial: u32,
    name: String,
    grade: u8,
    students: HashMap<u32, Student>,
    subjects: HashMap<String, Subject>
}


impl SchoolClass {
    fn new(id: u8, name: String, grade: u8 ) -> SchoolClass{

        SchoolClass{
            id: id,
            serial: 0,
            name: name,
            grade: grade,
            students: HashMap::new(),
            subjects: HashMap::new()

        }
    }
    
    fn addStudent(& mut self, name: String) {
        self.students.insert(self.serial, Student{id: self.serial, name: name});
        self.serial = self.serial + 1;
    }

    fn removeStudent(& mut self, id: u32) {
        self.students.remove(&id);
    }

    fn updateStudent(& mut self, id: u32, name: String) {
        let mut st = self.students.get_mut(&id);
        st.unwrap().name = name;
    }

    fn addSubject(& mut self, name: String, classroom: String) {
        self.subjects.insert(name.clone(), Subject{name: name, classroom: classroom});
    }
}


fn main() {
    // create class
    let mut class = SchoolClass::new(0, String::from("CherryClass"), 2);
    println!("{}", class.name);

    // add student
    class.addStudent(String::from("Evelyne"));
    println!("{}", class.students.len());

    class.addStudent(String::from("Coco"));
    println!("{}", class.students.len());

    // add subject
    class.addSubject(String::from("Math"), String::from("main building"));
    println!("{}", class.subjects.len());

    class.addSubject(String::from("English"), String::from("main building-2nd floor"));
    println!("{}", class.subjects.len());

    // update student name
    class.updateStudent(1, String::from("Elaine"));
    println!("{}", class.students.len());
    let st = class.students.get(&1);
    println!("{}", st.unwrap().name);

    // remove student
    class.removeStudent(1);
    println!("{}", class.students.len());

}
