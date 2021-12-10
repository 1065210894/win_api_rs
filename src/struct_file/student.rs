use crate::trait_file;

pub struct Student {
    pub name: String,
    pub age: i32,
}

impl Student {
    pub fn show_self_age(&self) {
        println!("我的年龄是:{}", &self.age)
    }
}

impl trait_file::show_me::ShowMe for Student {
    fn who_i_am(&self) {
        println!("我是:{}", self.name)
    }
}