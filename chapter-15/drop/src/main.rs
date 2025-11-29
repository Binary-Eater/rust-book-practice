struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/* std::mem::drop implementation */
//pub fn drop<T>(_x: T) {}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    //c.drop(); /* explicit use of destructor method; double free with implicit drop */
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
