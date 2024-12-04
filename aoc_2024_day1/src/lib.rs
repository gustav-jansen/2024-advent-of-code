use std::collections::HashMap;
use std::fs;
use std::error::Error;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Need filename as first argument.");
        }

        let file_path = args[1].clone();

        Config { file_path }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Need filename as first argument.");
        }

        Ok(Config::new(args))
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    let distance = find_distance(&content)?;
    println!("The distance is: {}", distance);

    let similarity = find_similarity(&content)?;
    println!("The similarity score is: {}", similarity);
    Ok(())
}

pub fn parse_content(content: &str) -> Result<(Vec<usize>, Vec<usize>), Box<dyn std::error::Error>> {
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();
        if words.len() != 2 {
            return Err(format!("Invalid input line: {}", line).into());
        }

        let left = words[0].parse::<usize>()?;
        let right = words[1].parse::<usize>()?;

        first_column.push(left);
        second_column.push(right);
    }
    Ok((first_column, second_column))
}

pub fn find_distance(content: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (mut c1, mut c2) = parse_content(content)?;

    c1.sort();
    c2.sort();

    Ok(c1.iter()
        .zip(c2.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum())
}

pub fn count_all_occurences(numbers: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    counts
}
pub fn find_similarity(content: &str) -> Result<usize, Box<dyn std::error::Error>> {
    let (left, right) = parse_content(content)?;

    let counts_from_left = count_all_occurences(&left);
    let counts_from_right = count_all_occurences(&right);

    let mut sum: usize = 0;
    for (key, left_value) in &counts_from_left {
        sum += key*left_value*counts_from_right.get(&key).copied().unwrap_or(0);
    }
    Ok(sum)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_of_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let content =  "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let result = find_distance(content)?;
        if result == 11 {
            Ok(())
        } else {
            return Err(format!("Distance is {}, expected 11", result).into());
        }
    }

    #[test]
    fn parsed_vectors_of_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let content =  "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let (col1, col2) = parse_content(content)?;
        if  col1 == [3,4,2,1,3,3] && col2 == [4,3,5,3,9,3] {
            Ok(())
        } else {
            return Err(format!("First column is {:?}, expected [3,4,2,1,3,3], \
                               while second column is {:?}, expected [4,3,5,3,9,4]", col1, col2).into());
        }
    }

    #[test]
    fn similarity_of_test_data() -> Result<(), Box<dyn std::error::Error>> {
        let content =  "\
3   4
4   3
2   5
1   3
3   9
3   3";
        let result = find_similarity(content)?;
        if result == 31 {
            Ok(())
        } else {
            return Err(format!("Similarity is {}, expected 31", result).into());
        }
    }

}
