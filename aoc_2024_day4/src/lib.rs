use std::fs;
use std::error::Error;

use config::Config;
use matrix::Matrix;

pub fn count_columns(content: &str) -> usize {
    if let Some(line) = content.lines().next() {
        line.len()
    } else {
        0
    }
}

pub fn count_rows(content: &str) -> usize {
    content.lines().count()
}

pub fn parse_content(content: &str) -> Result<Matrix<char>, Box<dyn Error>> {
    let number_of_columns = count_columns(content);
    let number_of_rows = count_rows(content);

    let mut matrix = Matrix::new(number_of_rows, number_of_columns, ' ');
    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            matrix.set(i, j, c)?;
        }
    }

    Ok(matrix)
}

pub fn has_word_lr(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if j + word.len() > matrix.ncols { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i, j+k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_rl(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if j + 1 < word.len() { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i, j-k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_ud(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + word.len() > matrix.nrows { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i+k, j).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_du(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + 1 < word.len() { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i-k, j).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_dur(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + 1 < word.len() { return false; }
    if j + word.len() > matrix.ncols { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i-k, j+k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_ddr(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + word.len() > matrix.nrows { return false; }
    if j + word.len() > matrix.ncols { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i+k, j+k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_dul(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + 1 < word.len() { return false; }
    if j + 1 < word.len() { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i-k, j-k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn has_word_ddl(word: &str, matrix: &Matrix<char>, i: usize, j: usize) -> bool {
    if i + word.len() > matrix.nrows { return false; }
    if j + 1 < word.len() { return false; }
    for (k,c) in word.char_indices() {
        let t = matrix.get(i+k, j-k).unwrap();
        if *t != c { return false; }
    }
    true
}

pub fn count_words(word: &str, matrix: &Matrix<char>) -> usize {
    let mut counter: usize = 0;

    for i in 0..matrix.nrows {
        for j in 0..matrix.ncols {
            if has_word_lr(word, matrix, i, j) { counter += 1; }
            if has_word_rl(word, matrix, i, j) { counter += 1; }
            if has_word_ud(word, matrix, i, j) { counter += 1; }
            if has_word_du(word, matrix, i, j) { counter += 1; }
            if has_word_dur(word, matrix, i, j) { counter += 1; }
            if has_word_ddr(word, matrix, i, j) { counter += 1; }
            if has_word_dul(word, matrix, i, j) { counter += 1; }
            if has_word_ddl(word, matrix, i, j) { counter += 1; }
        }
    }
    counter
}
pub fn count_crosses(word: &str, matrix: &Matrix<char>) -> usize {
    let mut counter: usize = 0;

    let second_char = word.chars().skip(1).next().unwrap();

    for i in 1..matrix.nrows-1 {
        for j in 1..matrix.ncols-1 {
            if *matrix.get(i,j).unwrap() != second_char { continue; }

            if ( has_word_dur(word, matrix, i+1, j-1) && has_word_ddr(word, matrix, i-1, j-1)) ||
                ( has_word_dur(word, matrix, i+1, j-1 ) && has_word_dul(word, matrix, i+1, j+1)) ||
                ( has_word_ddl(word, matrix, i-1, j+1) && has_word_ddr(word, matrix, i-1, j-1)) ||
                ( has_word_ddl(word, matrix, i-1, j+1) && has_word_dul(word, matrix, i+1, j+1)) {
                counter += 1;
            }

        }
    }
    counter
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let matrix: Matrix<char> = parse_content(&content)?;
    let number_of_words = count_words("XMAS", &matrix);

    println!("Occurence of XMAS: {number_of_words}");

    let number_of_crosses = count_crosses("MAS", &matrix);
    println!("Occurence of MAS crosses: {number_of_crosses}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_columns() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let ncols = count_columns(input);
        assert_eq!(10, ncols);
    }

    #[test]
    fn test_count_rows() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let nrows = count_rows(input);
        assert_eq!(10, nrows);
    }

    #[test]
    fn test_parse_content() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();
        assert_eq!(matrix.nrows, 10);
        assert_eq!(matrix.ncols, 10);

        let mut c = matrix.get(6,4).unwrap();
        assert_eq!(*c, 'S');

        c = matrix.get(7,7).unwrap();
        assert_eq!(*c, 'A');
    }

    #[test]
    fn test_has_word_lr() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_lr("XMAS", &matrix, 0,5));
        assert!(has_word_lr("XMAS", &matrix, 4,0));
        assert!(!has_word_lr("XMAS", &matrix, 9,9));
    }

    #[test]
    fn test_has_word_rl() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_rl("XMAS", &matrix, 1,4));
        assert!(has_word_rl("XMAS", &matrix, 4,6));
        assert!(!has_word_rl("XMAS", &matrix, 9,9));
    }

    #[test]
    fn test_has_word_ud() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_ud("XMAS", &matrix, 3,9));
        assert!(!has_word_ud("XMAS", &matrix, 9,9));
    }

    #[test]
    fn test_has_word_du() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_du("XMAS", &matrix, 4,6));
        assert!(has_word_du("XMAS", &matrix, 9,9));
        assert!(!has_word_du("XMAS", &matrix, 0,0));
    }

    #[test]
    fn test_has_word_dur() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_dur("XMAS", &matrix, 5,0));
        assert!(has_word_dur("XMAS", &matrix, 9,1));
        assert!(has_word_dur("XMAS", &matrix, 9,3));
        assert!(has_word_dur("XMAS", &matrix, 9,5));
        assert!(!has_word_dur("XMAS", &matrix, 0,0));
    }

    #[test]
    fn test_has_word_ddr() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_ddr("XMAS", &matrix, 0,4));
        assert!(!has_word_ddr("XMAS", &matrix, 0,0));
    }

    #[test]
    fn test_has_word_dul() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_dul("XMAS", &matrix, 5,6));
        assert!(has_word_dul("XMAS", &matrix, 9,3));
        assert!(has_word_dul("XMAS", &matrix, 9,5));
        assert!(has_word_dul("XMAS", &matrix, 9,9));
        assert!(!has_word_dul("XMAS", &matrix, 0,0));
    }

    #[test]
    fn test_has_word_ddl() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();

        assert!(has_word_ddl("XMAS", &matrix, 3,9));
        assert!(!has_word_ddl("XMAS", &matrix, 0,0));
    }

    #[test]
    fn test_count_words() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let matrix = parse_content(input).unwrap();
        let result = count_words("XMAS", &matrix);

        assert_eq!(result, 18);

    }
}
