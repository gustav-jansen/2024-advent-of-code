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

pub fn safe_changes(row: &Vec<usize>) -> bool {
    if row.len() < 2 { return true; }

    let increasing = row[0] < row[1];
    let mut result = true;
    for i in 1..row.len() {
        if row[i-1] < row[i] && increasing {
            result = result && row[i]-row[i-1] >= 1 && row[i]-row[i-1] <= 3;
        } else if row[i-1] > row[i] && !increasing {
            result = result && row[i-1] - row[i] >= 1 && row[i-1] - row[i] <= 3;
        } else {
            result = false;
        }
    }
    result
}

pub fn safe_changes_total(rows: &Vec<Vec<usize>>) -> Vec<bool> {
    rows.iter()
        .map(|a| safe_changes(a))
        .collect()
}

pub fn safe_reports(rows: &Vec<Vec<usize>>) -> usize {
    let result = safe_changes_total(rows);

    result.iter().filter(|&&x| x).count()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let rows = row_reader::read_rows_from_file(
        &config.file_path)?;

    let number_of_safe_reports = safe_reports(&rows);
    println!("Number of safe reports: {}", number_of_safe_reports);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_changes_total() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<Vec<usize>> = vec![
            [7,6,4,2,1].to_vec(),
            [1,2,7,8,9].to_vec(),
            [9,7,6,2,1].to_vec(),
            [1,3,2,4,5].to_vec(),
            [8,6,4,4,1].to_vec(),
            [1,3,6,7,9].to_vec()
        ];

        let actual = vec![true, false, false, false, false, true];
        let result = safe_changes_total(&input);

        if result == actual {
            Ok(())
        } else {
            Err(format!("Found {:?}, expected {:?}", result, actual).into())
        }
    }

    #[test]
    fn test_safe_reports() {
        let input: Vec<Vec<usize>> = vec![
            [7,6,4,2,1].to_vec(),
            [1,2,7,8,9].to_vec(),
            [9,7,6,2,1].to_vec(),
            [1,3,2,4,5].to_vec(),
            [8,6,4,4,1].to_vec(),
            [1,3,6,7,9].to_vec()
        ];

        assert_eq!(safe_reports(&input), 2);
    }

}

