fn main() {
    println!("Hello, world!");
    let zip = Counter::new().zip(Counter::new().skip(2));
    for (a, b) in zip {
        println!("{:?} {:?}", a, b);
    }

    // println!("zip value {:?}", zip.next().unwrap());
    let sum: Vec<_> = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .collect();
    println!("result {sum:#?}");
    let result: u32 = sum.iter().sum();
    println!("result {result}");

    assert!(result == 9);
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        // let counter = counter.iter();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_another() {
        // let zip= Counter::new()
        //     .zip(Counter::new().skip(1));
        // println!("zip value {zip}");

        let sum: Vec<_> = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .collect();
        println!("result {sum:#?}");
        let result: u32 = sum.iter().sum();
        assert!(result == 18);
    }
}
