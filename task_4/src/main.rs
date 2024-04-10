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

}