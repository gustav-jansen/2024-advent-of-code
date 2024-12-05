use std::collections::HashSet;
use std::fs;
use std::error::Error;
use config::Config;

#[derive(Debug,PartialEq)]
pub struct Update {
    pages: Vec<usize>,
    middle: usize,
}

impl Update {
    pub fn new( pages: Vec<usize> ) -> Self {
        let middle = pages[pages.len()/2 + 1];

        Update{ pages, middle }
    }
}

pub struct Queue {
    ordering: HashSet<(usize,usize)>,
    updates: Vec<Update>,
}

impl Queue {
    pub fn new() -> Self {
        Self { ordering: HashSet::new(), updates: Vec::new() }
    }
}

pub fn parse_ordering(content: &str) -> Result<HashSet<(usize,usize)>, Box<dyn Error>> {

    let mut ordering: HashSet<(usize, usize)> = HashSet::new();

    for line in content.lines().filter(|line| !line.trim().is_empty()) {
        let words: Vec<&str> = line.split('|').collect();

        if words.len() != 2 {
            return Err(format!("Line does not contain an ordering: {}", line).into());
        }

        ordering.insert((words[0].parse::<usize>()?, words[1].parse::<usize>()?));
    }

    Ok(ordering)
}

pub fn parse_updates(content: &str) -> Result<Vec<Update>, Box<dyn Error>> {

    let mut updates: Vec<Update> = Vec::new();
    for line in content.lines().filter(|line| !line.trim().is_empty()) {

        let words: Vec<&str> = line.split(',').collect();

        let mut pages: Vec<usize> = Vec::with_capacity(words.len());
        for word in words {
            pages.push(word.parse::<usize>()?);
        }

        updates.push( Update::new(pages) );
    }

    Ok(updates)
}

pub fn parse_content(content: &str) -> Result<Queue, Box<dyn Error>> {
    let mut parts = content.split("\n\n");

    let ordering_content = parts.next().ok_or("Unexpected content format")?;
    let updates_content = parts.next().ok_or("Unexpected content format")?;

    let ordering = parse_ordering(ordering_content)?;
    let updates = parse_updates(updates_content)?;


    Ok( Queue{ ordering, updates } )
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let queue = parse_content(&content)?;
    
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
use std::collections::HashSet;
    
    fn fill_hashset() -> HashSet<(usize,usize)> {
        let mut actual: HashSet<(usize,usize)> = HashSet::new();
        actual.insert((47,53));
        actual.insert((97,13));
        actual.insert((97,61));
        actual.insert((97,47));
        actual.insert((75,29));
        actual.insert((61,13));
        actual.insert((75,53));
        actual.insert((29,13));
        actual.insert((97,29));
        actual.insert((53,29));
        actual.insert((61,53));
        actual.insert((97,53));
        actual.insert((61,29));
        actual.insert((47,13));
        actual.insert((75,47));
        actual.insert((97,75));
        actual.insert((47,61));
        actual.insert((75,61));
        actual.insert((47,29));
        actual.insert((75,13));
        actual.insert((53,13));

        actual
    }

    fn fill_with_ordering() -> &'static str {
        "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13"
    }

    fn fill_with_updates() -> &'static str {
        "\
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }

    fn fill_updates() -> Vec<Update> {
        let mut updates: Vec<Update> = Vec::new();

        updates.push( Update::new([75,47,61,53,29].to_vec()));
        updates.push( Update::new([97,61,53,29,13].to_vec()));
        updates.push( Update::new([75,29,13      ].to_vec()));
        updates.push( Update::new([75,97,47,61,53].to_vec()));
        updates.push( Update::new([61,13,29      ].to_vec()));
        updates.push( Update::new([97,13,75,29,47].to_vec()));

        updates
    }

    #[test]
    fn test_parse_ordering() -> Result<(), Box<dyn std::error::Error>> {
        let input = fill_with_ordering();
        let ordering = parse_ordering(input)?;
        let actual = fill_hashset();

        if actual == ordering {
            Ok(())
        } else {
            Err(format!("Found: {:?}, expected: {:?}", ordering, actual).into())
        }
    }

    #[test]
    fn test_parse_updates() -> Result<(), Box<dyn std::error::Error>> {
        let input = fill_with_updates();
        let updates = parse_updates(input)?;
        let actual = fill_updates();

        if actual == updates {
            Ok(())
        } else {
            Err(format!("Found: {:?}, expected: {:?}", updates, actual).into())
        }
    }
}
