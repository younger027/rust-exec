extern crate young;

fn main() {
    let f = 98.6;
    let c = young::trans_f_to_c_temperature(f);
    println!("the f {f} to c {c}");

    let c = 37.0;
    let f = young::trans_c_to_f_temperature(c);
    println!("the c {c} to f {f}");


    let index_array = [1, 2, 3, 4, 7];
    for index in index_array {
        let result = young::produce_fbnq(index);
        println!("the fbnq index {index} is {result}")
    }

    young::christmas_loop()
}