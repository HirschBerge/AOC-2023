use crate::{gather_data, mul};

pub fn part1() -> u32 {
    let data = gather_data();
    let (_, data) = mul(data.as_str()).unwrap();
    data.into_iter().fold(0, |totes, instruct| {
            let value = instruct.multiply();
        totes + value
    })
}

#[cfg(test)]
mod tests {
    // use crate::*;

    use crate::mul;

    #[test]
    fn test_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (_, data) = mul(input).unwrap();
        let total = data.into_iter().fold(0, |totes, instruct| {
            let value = instruct.multiply();
            totes + value
        });
        assert_eq!(total, 161);
        // assert_eq!(1,1);
    }
}
