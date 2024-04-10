struct Cats {
    name: String,
    sound: String
}

struct Dogs {
    name: String,
    sound: String
}

struct Birds {
    name: String,
    sound: String
}

trait Animal {
    fn talk(&self);
 }

 impl Animal for Cats {
   fn talk(&self) {
     println!("talk {}", self.sound);
   }
 }
 
 impl Animal for Dogs {
   fn talk(&self) {
    println!("talk {}", self.sound);
   }
 }

 impl Animal for Birds {
    fn talk(&self) {
        println!("talk {}", self.sound);
    }
  }


enum Animals {
    Cats(String, String),
    Dogs(String, String),
    Birds(String, String)
}

fn sound(animal: &Animals) {
    match animal {
        Animals::Cats(name, sound) => println!("sound '{}'.", sound),
        Animals::Dogs(name, sound) => println!("sound '{}'.", sound),
        Animals::Birds(name, sound) => println!("sound '{}'.", sound),
    }
}

trait AddTrait<T> {
    fn add(a1:T, a2:T) -> T;
}

struct Integer {
    value: i32
}

struct Float {
    value: f32
}

impl AddTrait<Integer> for Integer {
    fn add(a1: Integer, a2: Integer) -> Integer{
        Integer{value: a1.value + a2.value}
    }
}

impl AddTrait<Float> for Float {
    fn add(a1: Float, a2: Float) -> Float{
        Float{value: a1.value + a2.value}
    }
}


fn main() {
    // use enum
    let black_cat = Animals::Cats(String::from("xiaohei"), String::from("miaomiao"));
    let good_dog = Animals::Dogs(String::from("wangcai"), String::from("barkbark"));
    let yingwu_bird = Animals::Birds(String::from("polly"), String::from("aaaaaa"));

    let mut vec = Vec::new();
    vec.push(&black_cat);
    vec.push(&good_dog);
    vec.push(&yingwu_bird);

    for x in vec {
        sound(x);
    }

    // use trait
    let black_cat = Cats{name: String::from("xiaohei"), sound: String::from("miaomiao")};
    let good_dog = Dogs{name: String::from("wangcai"), sound: String::from("barkbark")};
    let yingwu_bird = Birds{name: String::from("polly"), sound: String::from("aaaaaa")};

    let ans: Vec<&dyn Animal> = vec![&black_cat, &good_dog, &yingwu_bird];

    for x in ans {
        x.talk();
    }

    // impl add
    let int_10 = Integer{value:10};
    let int_20 = Integer{value:20};
    let int_30 = Integer::add(int_10, int_20);
    println!("{}", int_30.value);

    let float_10_23123 = Float{value:10.23123};
    let float_20_123573 = Float{value:20.123573};
    let float_30 = Float::add(float_10_23123, float_20_123573);
    println!("{}", float_30.value);

}