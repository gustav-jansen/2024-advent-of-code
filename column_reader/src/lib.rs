use std::fs;

pub fn parse_content(content: &str, number_of_columns: usize)
        -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {

    let mut columns: Vec<Vec<usize>> = vec![Vec::new(); number_of_columns];

    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() != number_of_columns {
            return Err(format!("Invalid input line: {}", line).into());
        }

        words.iter()
            .enumerate()
            .try_for_each(|(i, word)| -> Result<(), Box<dyn std::error::Error>> {
                let item = word.parse::<usize>()?;
                columns[i].push(item);
                Ok(())
            })?;
    }
    Ok(columns)
}

pub fn read_columns_from_file(filename: &str, number_of_columns: usize)
        -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let result = parse_content(&content, number_of_columns)?;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content() -> Result<(), Box<dyn std::error::Error>> {
        let content =  "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let actual = vec![
            [7,1,9,1,8,1],
            [6,2,7,3,6,3],
            [4,7,6,2,4,6],
            [2,8,2,4,4,7],
            [1,9,1,5,1,9]
        ];
        let columns = parse_content(content, actual.len())?;
        assert_eq!(columns.len(), actual.len());


        if columns
            .iter()
            .zip(actual.iter())
            .all(|(a,b)| a == b)
        {
            Ok(())
        } else {
            let mut s: String = "".to_string();
            let _ = columns.iter()
                .zip(actual.iter())
                .for_each(|(a, b)| {
                    if a != b {
                        s += &format!("Found {:?}, expected {:?}, ", a, b);
                    }
                });

            s.pop();s.pop();
            return Err(s.into());
        }
    }
}
