fn main() {
    let mut sum = 0;
    for n in 1000000000i64..9999999999i64 {
        if has_property(n) {
            println!("{:?}", n);
            sum += n
        }
    }
    println!("sum = {:?}", sum);
    //println!("{:?}", has_property(1234567890));
    //println!("{:?}", has_property(1406357289));
}

fn has_property(n: i64) -> bool {
    let arr = as_array(n);
    if !is_pandigital(&arr) {
        return false;
    }
    //println!("{:?}", &arr);
    let dividers = vec![2, 3, 5, 7, 11, 13, 17];
    let mut divides_all = true;
    for i in 0..dividers.len() {
        let ddd = arr[1+i] * 100 + arr[2+i] * 10 + arr[3+i];
        let divides = ddd % dividers[i] == 0;
        //println!("{:?}, {:?} -> {:?}", ddd, dividers[i], divides);
        divides_all &= divides;
    }
    return divides_all;
}

fn as_array(n: i64) -> Vec<i64> {
    let mut tmp = n;
    let mut ds = vec![0; 10];
    for i in (0..10).rev() {
        ds[i] = tmp % 10;
        tmp /= 10;
    } 
    return ds
}

fn is_pandigital(arr: &Vec<i64>) -> bool {
    let pandigits: u16 = 0b0000_0011_1111_1111;
    let one: u16 = 0b0000_0000_0000_0001;
    let mut digits = 0b0000_0000_0000_0000;
    for i in 0..10 {
        digits |= one << arr[i] 
    }
    return digits == pandigits
}