pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//test_1 相互转换摄氏与华氏温度

//摄氏度 = (华氏度 - 32°F) ÷ 1.8 （c=5/9×(f-32)）
pub fn trans_f_to_c_temperature(f: f64) -> f64 {
    let w: f64 = 5.0/9.0;
    return w*(f-32.0);
}


//华氏度 = 32°F+ 摄氏度 × 1.8 （f=9/5×c+32）
pub fn trans_c_to_f_temperature(c: f64) -> f64 {
    let w: f64 = (9.0/5.0) as f64;
    return w*c+32.0;
}

//生成第 n 个斐波那契数
pub fn produce_fbnq(index: i64) -> i64 {
    if index == 0 {
        return 0;
    }

    if index == 1 {
        return 1;
    }


    return produce_fbnq(index-1) + produce_fbnq(index-2);
}


pub fn christmas_loop() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"];
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[i]);
        for j in (0..=i).rev(){
            if j == 0 && i != 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }
        println!();
    }
    
    
}

pub fn owner_ship() {
    // let mut s = String::from("hello");
    // s.push_str(", world!");
    // println!("{}", s)

    // let s = String::from("hello");

    // check_use_space_for_string(s.clone());
    // println!("{}", s);

    // let x =  5;
    // check_use_space_for_i32(x);
    // println!("{}", x);

    //reference value, do not have ownership
    // let s1 = String::from("hello");
    // let len = caculcate_length(&s1);
    // println!("the length of {} is {}", s1, len);

    // let mut s = String::from("hello");
    // change(&mut s); //notice this place must add mut
    // println!("the s is {}", s);

    let mut s = String::from("hello");

    //在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //can not run,just have one reference in use space
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    //case 2: this worked
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;

    //case 3: when e3 init, r1 and r2 not use after code
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);



}

fn caculcate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}
fn check_use_space_for_string(s : String) {
    println!("{}", s);
}

fn check_use_space_for_i32(s : i32) {
    println!("{}", s);
}

//悬挂指针，返回引用的话， s出了作用域，堆上的内存就会被销毁，变为悬挂指针
fn return_string() -> String {
    let s = String::from("hello");
    s //return string not return reference of string
}

//错误示例
fn return_string_reference() -> &String {
    let s = String::from("hello");
    &s //return string not return reference of string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
