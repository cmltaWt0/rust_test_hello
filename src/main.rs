use std::env::args;


enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

type Num = VeryVerboseEnumOfThingsToDoWithNumbers;



fn main() {
    let _s = "色な ばか";
    let mut s = String::new();
    s.push_str("string: &str");
    let _t2 = s.split(" ").collect::<Vec<&str>>();
    let t = _s.split(" ").collect::<Vec<&str>>();
    println!("{}", _t2[0]);
    println!("{}", _t2[0]);
    
    for i in &_t2 {
        println!("{}", i);
    }
    println!("{}", _t2[0]);


    println!("Len {}", s.chars().count());
    test(_s);
    test(&s);


    struct User {
        username: String,
        age: i32,
    }

    let u = User{
        username: "Max".to_string(),
        age: 35
    };

    impl User{
        fn present(&self) -> String {
            format!("{} {}", self.username, self.age)
        }

        fn die(&self) {
            println!("Die");
        }
    }

    println!("{}", u.present());
    u.die();
    u.die();


    enum Room {
        Kitchen(i32),
        Bed(i32),
    }

    let t = Room::Bed(2);

    let v = match t {
        Room::Bed(5) => 5,
        _ => 0,
    } + 10;

    println!("{}", v);

    let m = Num::Add;
    let m2 = Num::Subtract;

    println!("New::{}", m.run(1, 2));
    println!("New::{}", m2.run(1, 2));


    let visitors: [String; 5] = [
        "Max".to_string(),
        "Ilona".to_string(),
        "Serg".to_string(),
        "Vitali".to_string(),
        "Milana".to_string(),
    ];

    struct User2 {
        name: String,
    }
    
    impl User2 {
        fn is_start_from_w(&self) -> bool {
            if self.name.starts_with("I") {
                return true;
            }
            false
        }

        fn is_need_to_say_hello(&self) -> bool {
            let args: Vec<String> = args().collect();
            for i in args {
                if self.name.eq(&i) {
                    return true;
                }
            }
            false
        }
    }

    for name in visitors.iter() {
        let u = User2 {
            name: name.to_string(),
        };

        match u.is_start_from_w() && u.is_need_to_say_hello() {
            true => println!("Hello {}", &name),
            false => {}
        }
        
    }

    for name in args() {
        if let Some(c) = arg.chars().next() {
            match name {
                'i' | 'I' => println!("Hello {}", name),
                _ => {}
            }
        }
    }

}

fn say_hello(name: &str) {
    println!("Hello {}", name)
}



fn test(s: &str) {
    println!("{}", s);
}