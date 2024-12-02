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

pub fn safe_changes(vec_a: &Vec<usize>, vec_b: &Vec<usize>) -> Vec<bool> {
    let result: Vec<bool> = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a,b)|
             if a > b {
                 a-b >= 1 && a-b <= 3
             } else {
                 b-a >= 1 && b-a <= 3
             })
    .collect();

    result
}

pub fn safe_changes_with_conditions(vec_a: &Vec<usize>, vec_b: &Vec<usize>, increasing: &Vec<bool>) -> Vec<bool> {
    let result: Vec<bool> = vec_a.iter()
        .zip(vec_b.iter())
        .zip(increasing.iter())
        .map(|((a,b), &increases)|
             if a < b && increases {
                 b-a >=1 && b-a <= 3
             } else if a > b && !increases {
                a-b >= 1 && a-b <= 3
             } else {
                 false
             })
    .collect();

    result
}

pub fn safe_changes_increasing(vec_a: &Vec<usize>, vec_b: &Vec<usize>) -> Vec<bool> {
    let result: Vec<bool> = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a,b)|
             if a < b {
                 b-a >= 1 && b-a <= 3
             } else {false}
             )
    .collect();

    result
}
pub fn safe_changes_decreasing(vec_a: &Vec<usize>, vec_b: &Vec<usize>) -> Vec<bool> {
    let result: Vec<bool> = vec_a.iter()
        .zip(vec_b.iter())
        .map(|(a,b)|
             if a > b {
                 a-b >= 1 && a-b <= 3
             } else {false}
             )
    .collect();

    result
}
pub fn safe_changes_total(columns: &Vec<Vec<usize>>) -> Vec<bool> {
    if columns.len() == 0 {
        let result: Vec<bool> = vec![true];
        return result;
    }

    let mut result: Vec<bool> = vec![true; columns[0].len()];
    if columns.len() == 1 {
        return result;
    }

    let increasing = safe_changes_increasing(&columns[1], &columns[0]);
    for i in 1..columns.len()-1 {
        for (a, b) in result.iter_mut().zip(safe_changes_with_conditions(&columns[i], &columns[i-1], &increasing)) {
            *a = *a && b;
        }
    }
    result
}

pub fn safe_reports(columns: &Vec<Vec<usize>>) -> usize {
    let result = safe_changes_total(columns);

    result.iter().filter(|&&x| x).count()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let columns = column_reader::read_columns_from_file(
        &config.file_path, 5)?;

    let number_of_safe_reports = safe_reports(&columns);
    println!("Number of safe reports: {}", number_of_safe_reports);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_changes_for_identical() -> Result<(), Box<dyn std::error::Error>> {
        let vec_a: Vec<usize> = vec![1,2,3,4,5];
        let vec_b: Vec<usize> = vec![1,2,3,4,5];

        let result: Vec<bool> = safe_changes(&vec_a, &vec_b);

        let actual = vec![false,false,false,false,false];
        assert_eq!(result.len(), actual.len());

        if result == actual {
            Ok(())
        } else {
            return Err(format!("Found {:?}, expected {:?}", result, actual).into());
        }
    }

    #[test]
    fn test_safe_changes_for_greater() -> Result<(), Box<dyn std::error::Error>> {
        let vec_a: Vec<usize> = vec![1,2,3,4,5];
        let vec_b: Vec<usize> = vec![2,4,6,9,8];

        let result: Vec<bool> = safe_changes(&vec_a, &vec_b);

        let actual = vec![true,true,true,false,true];
        assert_eq!(result.len(), actual.len());

        if result == actual {
            Ok(())
        } else {
            return Err(format!("Found {:?}, expected {:?}", result, actual).into());
        }
    }

    #[test]
    fn test_safe_changes_for_lesser() -> Result<(), Box<dyn std::error::Error>> {
        let vec_a: Vec<usize> = vec![7,2,3,4,5];
        let vec_b: Vec<usize> = vec![2,1,1,2,2];

        let result: Vec<bool> = safe_changes(&vec_a, &vec_b);

        let actual = vec![false,true,true,true,true];
        assert_eq!(result.len(), actual.len());

        if result == actual {
            Ok(())
        } else {
            return Err(format!("Found {:?}, expected {:?}", result, actual).into());
        }
    }

    #[test]
    fn test_safe_changes_for_mixed() -> Result<(), Box<dyn std::error::Error>> {
        let vec_a: Vec<usize> = vec![7,2,3,4,5,7,8,9,15];
        let vec_b: Vec<usize> = vec![2,1,4,2,2,11,10,12,15];

        let result: Vec<bool> = safe_changes(&vec_a, &vec_b);

        let actual = vec![false,true,true,true,true,false,true,true,false];
        assert_eq!(result.len(), actual.len());

        if result == actual {
            Ok(())
        } else {
            return Err(format!("Found {:?}, expected {:?}", result, actual).into());
        }
    }

    #[test]
    fn test_safe_changes_total() -> Result<(), Box<dyn std::error::Error>> {
        let input: Vec<Vec<usize>> = vec![
            [7,1,9,1,8,1].to_vec(),
            [6,2,7,3,6,3].to_vec(),
            [4,7,6,2,4,6].to_vec(),
            [2,8,2,4,4,7].to_vec(),
            [1,9,1,5,1,9].to_vec()
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
            [7,1,9,1,8,1].to_vec(),
            [6,2,7,3,6,3].to_vec(),
            [4,7,6,2,4,6].to_vec(),
            [2,8,2,4,4,7].to_vec(),
            [1,9,1,5,1,9].to_vec()
        ];

        assert_eq!(safe_reports(&input), 2);
    }

}

