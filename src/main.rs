use std::mem::drop;

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("My random stuff"),
    };


    let d = CustomSmartPointer {
        data: String::from("Other random stuff"),
    };

    println!("CustomSmartPointers Created!\n{:#?}", d);
    drop(c);
    println!("c was dropped before d was dropped and before main exited")
}
