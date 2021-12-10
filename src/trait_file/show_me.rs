pub trait ShowMe {
    fn who_i_am(&self);
}

pub fn show_name<T: ShowMe>(object: &T) {
    object.who_i_am()
}