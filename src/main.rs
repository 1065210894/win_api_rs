mod struct_file;
mod trait_file;
mod util;

use rand::AsByteSliceMut;

fn main() {
    let name = String::from("小明");
    let age = 18;
    let mut x_ming = struct_file::student::Student {name, age};
    trait_file::show_me::show_name(&x_ming);
    x_ming.show_self_age();
    util::student_util::end_life(&mut x_ming);
    println!("小明已死:{} {}", x_ming.age, x_ming.name)
}
