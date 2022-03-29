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
}
