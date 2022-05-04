#![allow(unused)]
use std::collections::HashMap;
//https://doc.rust-lang.ru/book/appendix-02-operators.html

fn num_as_roman(num: i32) -> String {
    let mut result = "".to_string();
    let v = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    //
    // let rm: HashMap<i32, _> = HashMap::from_iter([]);
    let mut repeat = 0i32;
    let mut count = 0i32;
    let mut num = num;
    for (i, s) in &v {
        repeat = num / i;
        if repeat < 4 {
            let str1 = (*s).repeat(repeat as usize);
            result.push_str(&str1);
            num -= i * repeat;
        } else {
            //Debug
            // dbg!(repeat);
        }
        count += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(num_as_roman(4), "IV");
        assert_eq!(num_as_roman(1474), "MCDLXXIV");
        // assert_eq!(num_as_roman(182), "CLXXXII");
        // assert_eq!(num_as_roman(1990), "MCMXC");
        // assert_eq!(num_as_roman(1875), "MDCCCLXXV");
    }
}

fn num_as_roman2(num: i32) -> String {
    let mut result = "".to_string();
    let rs = "MDCLXVI";
    let v = vec![1000, 500, 100, 50, 10, 5, 1];
    //HashMap::from_iter([("one", 1), ("two", 2), ("three", 3)]);
    let rm: HashMap<_, _> = HashMap::from_iter(rs.chars().enumerate());
    let mut repeat = 0i32;
    let mut count = 0i32;
    let mut num = num;
    for i in &v {
        repeat = num / i;
        if repeat < 4 {
            let str1 = rm
                .get(&(count as usize))
                .unwrap()
                .to_string()
                .repeat(repeat as usize);
            result.push_str(&str1);
            num -= i * repeat;
        } else {
            let str1 = rm
                .get(&(count as usize))
                .unwrap()
                .to_string()
                .repeat(repeat as usize);
            result.push_str(&str1);
            num -= i * repeat;
            dbg!(repeat);
        }
        count += 1;
    }
    // let sdfsdf = rm.get(&0);
    // dbg!(sdfsdf.unwrap());

    result
}

fn num_as_roman1(num: i32) -> String {
    // match num {
    //     (x) => String::from("")
    // }
    // M:1000, D:500, C:100, L:50, X:10, V:5, I:1
    let v = vec![1000, 500, 100, 50, 10, 5, 1];
    let mut rm: HashMap<i32, char> = HashMap::new();
    let rs = "MDCLXVI";
    let mut result = "".to_string();
    let mut count = 0;
    for ch in rs.chars() {
        rm.insert(v[count], ch);
        count += 1;
    }
    // dbg!(rm.get(&50));
    count = 0;
    let mut num = num;
    for i in &v {
        let rem_dev = num % i;
        let mut repeat = 0;
        repeat = num / i;
        let t: String;
        if num - i > i + v[count + 1] {
            dbg!(repeat);
            let mut prev = rm.get(&(v[count + 1])).unwrap().to_string();
            prev.push_str(&rm.get(&(i)).unwrap().to_string());
            t = prev;
        } else {
            t = rm.get(&(i)).unwrap().to_string().repeat(repeat as usize)
        }
        // num - i > i + next i (990 - 500 > i + 100)
        if rem_dev == num && i >= &1 {
            continue;
        };
        dbg!(num);
        dbg!(i);
        dbg!(rem_dev);
        dbg!(repeat);
        result.push_str(&t);
        num = rem_dev;
        count += 1;
        // dbg!(&result);
    }
    result

    // let v = vec![1,5,10,50,100,500].reserve();
    // let v = vec![1, 5, 10, 50, 100, 500];
    // if num <= 0 {
    //     String::from("")
    // } else {
    // }
}
