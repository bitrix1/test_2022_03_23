fn solution(s: &str) -> String {
    s.chars()
        .flat_map(|c| {
            if c.is_ascii_uppercase() {
                vec![' ', c]
            } else {
                vec![c]
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}

// fn solution(s: &str) -> String {
//     // let hello = &s[0..5];
//     let mut s_vec: Vec<char> = "".chars().collect();
//     for (_, val) in s.chars().enumerate() {
//         if val.to_string() != val.to_uppercase().to_string() {
//             // my_vec[i] = val;
//             s_vec.append(&mut vec![val]);
//         } else if val.to_string() == val.to_uppercase().to_string() {
//             s_vec.append(&mut vec![' ', val]);
//         }
//     }
//     println!("{:?}", s_vec);
//     s_vec.into_iter().collect()
// }

// fn solution(s: &str) -> String {
//     let mut res = String::new();
//     for c in s.chars() {
//         if c.is_uppercase() {
//             res.push(' ');
//         }
//         res.push(c);
//     }
//     res
// }
