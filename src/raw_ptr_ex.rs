#![allow(unused)]

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fn1() {
        let mut num = 5;
        //Raw Pointer
        let r1 = &num as *const i32;
        //Raw Pointer
        let r2 = &mut num as *mut i32;
        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {:?}", r2);
            let address = 0x3ca1aff444usize;
            //Raw Pointer
            let r3 = address as *const i32;
            //r3 is: 0x3ca1aff444
            println!("r3 is: {:?}", r3);
            //Разыменовываем сырой указатель и ошибка exit code: 0xc0000005, STATUS_ACCESS_VIOLATION.
            println!("r3 is: {:?}", *r3);
        }
    }
    #[test]
    fn fn2() {
        let abcd = String::from("abcdфыва😂");
        for (index, ch) in abcd.char_indices() {
            let slice = &abcd[index..index + ch.len_utf8()];
            // let char2: Vec<char> = slice.iter().map(|b| *b as char).collect::<Vec<_>>();
            // let s = "Hello world!";
            // let char_vec: Vec<char> = s.chars().collect();
            let mut v1 = Vec::from(slice);
            let mut v2 = &v1[0];
            println!("{:?}", v1);
            v1[0] = 100;
            v2 = &0;
            println!("{:?}", v1);
            println!("{:?}", &slice[0..ch.len_utf8()]);
            // println!("{:?}, len: {:?}", slice, ch.len_utf8());
        }
        dbg!(println!("{:?}", abcd));
    }
}
