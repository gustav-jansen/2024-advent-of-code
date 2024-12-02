use std::fs;

pub fn parse_content_by_rows(content: &str)
        -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {

    let mut rows: Vec<Vec<usize>> = Vec::new();

    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        let words: Vec<&str> = line.split_whitespace().collect();
        let items: Vec<usize> = words.iter()
            .map(|s| s.parse::<usize>())
            .collect::<Result<_,_>>()?;

        rows.push(items);

    }
    Ok(rows)
}

pub fn read_rows_from_file(filename: &str)
        -> Result<Vec<Vec<usize>>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?;
    let result = parse_content_by_rows(&content)?;

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
            [7,6,4,2,1],
            [1,2,7,8,9],
            [9,7,6,2,1],
            [1,3,2,4,5],
            [8,6,4,4,1],
            [1,3,6,7,9]
        ];
        let rows = parse_content_by_rows(content)?;
        assert_eq!(rows.len(), actual.len());


        if rows
            .iter()
            .zip(actual.iter())
            .all(|(a,b)| a == b)
        {
            Ok(())
        } else {
            let mut s: String = "".to_string();
            let _ = rows.iter()
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
