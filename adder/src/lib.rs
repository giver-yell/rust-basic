// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[derive(Debug)]
// struct Rectagle {
//     width: u32,
//     height: u32,
// }

// impl Rectagle {
//     fn can_hold(&self, other: &Rectagle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// pub fn greeting(name: &str) -> String {
//     // format!("Hello {}!", name)
//     String::from("Hello!")
// }

// pub struct Guess {
//     value: i32,
// }

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 {
//             panic!("Guess value must be greater than 1, got {}.", value);
//         } else if value > 100 {
//             panic!("Guess value must be less than 100, got {}.", value)
//         }

//         Guess { value }
//     }
// }

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn exploration() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectagle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectagle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));

    //     #[test]
    //     fn smaller_cannot_hold_larger() {
    //         let larger = Rectagle {
    //             width: 8,
    //             height: 7,
    //         };
    //         let smaller = Rectagle {
    //             width: 5,
    //             height: 1,
    //         };

    //         assert!(!smaller.can_hold(&larger));
    //     }
    // }

    // Adding Custom Failure Messages
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`",
    //         result
    //     );
    // }

    // #[test]
    // #[should_panic]
    // fn greater_than_100() {
    //     Guess::new(200);
    // }

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}
