use crate::struct_file::student::Student;

pub fn end_life(student:&mut Student) {
    student.age = -1;
    student.name = String::from("nothing")
}