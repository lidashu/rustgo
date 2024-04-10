fn foo(s: String) -> String {
    println!("{s}");
    // transfer back
    s
}

fn foo_immut(s: &String) {
    println!("in fn foo: {s}");
}

fn foo_mut(s: &mut String) {
    s.push_str(" You are batman.");
    }


fn main() {
    // 复制类型
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}");

    // String转移所有权
    let s1 = String::from("I am a superman.");
    let s2 = s1;
    // wrong
    // println!("{s1}");
    println!("{s2}");

    // 转移给了函数
    let s1 = foo(s2);
    // wrong
    println!("{s1}"); 

    // 引用
    let mut a = 10u32;

    let b = &mut a;
    *b = 20;
    // can't borrow
    // println!("{a}");
    println!("{b}");

    let c = &a;
    // 不能引用
    //println!("{b}");
    // 所有权在c
    println!("{c}"); //


    let mut a = 10u32;
    let r1 = &mut a;
    let r2 = r1;
    // wrong 所有权不在了
    // println!("{r1}")
    println!("{r2}");


    // 函数中的引用
    let s1 = String::from("I am a superman.");
    foo_immut(&s1);
    println!("{s1}");

    // 可变引用
    let mut s1 = String::from("I am a superman.");
    println!("{s1}");
    foo_mut(&mut s1);
    println!("{s1}");
}