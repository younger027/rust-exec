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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
