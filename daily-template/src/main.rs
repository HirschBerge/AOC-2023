use {{crate_name}}::*;

fn main() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_one() {
        assert_eq!(1, 1);
    }
}
