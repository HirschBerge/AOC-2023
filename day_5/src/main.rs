fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = aochelpers::get_daily_input(5, 2023)?;
    for line in data.lines() {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> Result<(), Box<dyn Error>> {
        let data = "".to_string();
        assert_eq!("13".to_string(), insert_name(data)?);
        Ok(())
    }
}
