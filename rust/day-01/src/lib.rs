pub fn process_part1(input: &str) -> String {
  let result = input
      .split("\n\n")
      .map(|load| {
          load
            .lines()
            .map(|item| item.parse::<u32>().unwrap())
            .sum::<u32>()
      })
      .max()
      .unwrap();
  result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
      let result: String = process_part1(input);
      assert_eq!(result, "24000");
    }
}
