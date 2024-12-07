use std::fs;
use std::error::Error;

use config::Config;
use matrix::Matrix;

static BLOCKED: char = '#';
static START: char = '^';
static VISITED: char = '#';

#[repr(usize)]
#[derive(PartialEq,Debug,Clone, Copy)]
enum Orientation {
    Up = 0,
    Right,
    Down,
    Left,
}


impl Orientation {

    fn next(&mut self) {
        *self = match *self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        };
    }

}

#[derive(Debug,PartialEq)]
pub struct Map {
    pub rows: Vec<Vec<usize>>,
    pub cols: Vec<Vec<usize>>,
    pub starting_position: (usize, usize),

    height: usize,
    width: usize,
}

impl Map {
    pub fn new(rows: Vec<Vec<usize>>, cols: Vec<Vec<usize>>, starting_position: (usize, usize)) -> Self {

        let height = rows.len();
        let width = cols.len();

        Map { rows, cols, starting_position, height, width }
    }

    pub fn parse_input(input: &str) -> Self {
        let nrows = input.lines()
            .count();

        let ncols = input.lines()
            .next()
            .unwrap_or("")
            .chars()
            .count();

        let mut rows: Vec<Vec<usize>> = vec![Vec::new(); nrows];
        let mut cols: Vec<Vec<usize>> = vec![Vec::new(); ncols];

        let mut starting_position = (0,0);

        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == BLOCKED {
                    rows[row].push(col);
                    cols[col].push(row);
                } else if c == START {
                    starting_position = (row, col);
                }
            }
        }

        Map::new(rows, cols, starting_position)
    }

    pub fn is_inside(&self, position: (usize, usize)) -> bool {
        let (row, col) = position;

        row < self.height && col < self.width
    }

}

#[derive(Debug,PartialEq)]
struct Path {
    pub map: Map,
    pub current_position: (usize, usize),
    pub free_space: Matrix<(usize,usize,usize,usize)>,
    pub visited: Matrix<bool>,
    pub orientation: Orientation,
}

type WalkFn = fn(&mut Path, usize, usize);
impl Path {
    pub fn new(map: Map) -> Path {

        let current_position = map.starting_position;
        let free_space = Path::setup_free_space(&map);

        let mut visited: Matrix<bool> = Matrix::new(map.height, map.width, false);
        visited.set(current_position.0, current_position.1, true).unwrap();

        Path { map, current_position, free_space, visited, orientation: Orientation::Up }
    }

    fn walk_up(&mut self, row: usize, col: usize) {
        let npos = self.free_space.get(row, col).unwrap().0;
        println!("Walk up: {}, {}, {}", row, col, npos);

        for walk in (row-npos+1..row).rev() {
            println!("UP: walk: {}", walk);
            self.visited.set(walk, col, true).unwrap();
            self.current_position = (walk, col);
        }

        if self.current_position.0 == 0 {
            self.current_position.0 = self.map.height;
        }
    }

    fn walk_right(&mut self, row: usize, col: usize) {
        let npos = self.free_space.get(row, col).unwrap().1;
        println!("Walk right: {}, {}, {}", row, col, npos);

        for walk in col+1..col+npos {
            self.visited.set(row, walk, true).unwrap();
            self.current_position = (row, walk);
        }

        if self.current_position.1 == self.map.width - 1 {
            self.current_position.1 += 1;
        }
    }

    fn walk_down(&mut self, row: usize, col: usize) {
        let npos = self.free_space.get(row, col).unwrap().2;
        println!("Walk down: {}, {}, {}", row, col, npos);

        for walk in row+1..row+npos {
            self.visited.set(walk, col, true).unwrap();
            self.current_position = (walk, col);
        }

        if self.current_position.0 == self.map.height - 1 {
            self.current_position.0 += 1;
        }
    }

    fn walk_left(&mut self, row: usize, col: usize) {
        let npos = self.free_space.get(row, col).unwrap().3;
        println!("Walk left: {}, {}, {}", row, col, npos);

        for walk in (col-npos+1..col).rev() {
            self.visited.set(row, walk, true).unwrap();
            self.current_position = (row, walk);
        }

        if self.current_position.1 == 0 {
            self.current_position.1 = self.map.width;
        }
    }

    fn walk_funcs() -> &'static [WalkFn] {
        &[
            Path::walk_up,
            Path::walk_right,
            Path::walk_down,
            Path::walk_left,
        ]
    }

    fn walk(&mut self) {

        let row = self.current_position.0;
        let col = self.current_position.1;

        let funcs = Path::walk_funcs();
        let index = self.orientation as usize;

        if let Some(&walk_fn) = funcs.get(index) {
            walk_fn(self, row, col);
        } else {
            panic!("Invalid Orientation index!");
        }
    }
    pub fn get_up_free_space(map: &Map) -> Matrix<usize> {
        let mut free_space: Matrix<usize> = Matrix::new ( map.height, map.width, 0 );

        let mut next_block_idx;
        let mut val;

        for i in 0..map.cols.len() {
            next_block_idx = 0;
            for j in 0..map.rows.len() {
                // On a blocked square
                if next_block_idx != map.cols[i].len() {
                    if j == map.cols[i][next_block_idx] {
                        free_space.set(j, i, 0).unwrap();
                        next_block_idx += 1;
                        continue
                    }
                }

                if next_block_idx == 0 { // Before first blocked square in column
                    val = j + 1;
                } else {
                    val = j - map.cols[i][next_block_idx-1];
                }
                
                free_space.set(j, i, val).unwrap();
            }
        }
        free_space
    }

    pub fn get_right_free_space(map: &Map) -> Matrix<usize> {
        let mut free_space: Matrix<usize> = Matrix::new ( map.height, map.width, 0 );

        let mut next_block_idx;
        let mut val;

        let max_val = map.width;
        for i in 0..map.rows.len() {
            next_block_idx = 0;
            for j in 0..map.cols.len() {
                if next_block_idx == map.rows[i].len() { // No more blocked squares
                    val = max_val - j;
                } else {
                    val = map.rows[i][next_block_idx] - j;
                }

                // On a blocked square
                if val == 0 { next_block_idx += 1; }

                free_space.set(i, j, val).unwrap();
            }
        }
        free_space
    }

    pub fn get_down_free_space(map: &Map) -> Matrix<usize> {
        let mut free_space: Matrix<usize> = Matrix::new( map.height, map.width, 0 );

        let mut next_block_idx;
        let mut val;

        let max_val = map.height;
        for i in 0..map.cols.len() {
            next_block_idx = 0;
            for j in 0..map.rows.len() {
                if next_block_idx == map.cols[i].len() { // No more blocked squares
                    val = max_val - j;
                } else {
                    val = map.cols[i][next_block_idx] - j;
                }

                // On a blocked square
                if val == 0 { next_block_idx += 1; }

                free_space.set(j, i, val).unwrap();
            }
        }
        free_space
    }

    pub fn get_left_free_space(map: &Map) -> Matrix<usize> {
        let mut free_space: Matrix<usize> = Matrix::new ( map.height, map.width, 0 );

        let mut next_block_idx;
        let mut val;

        for i in 0..map.rows.len() {
            next_block_idx = 0;
            for j in 0..map.cols.len() {
                // On a blocked square
                if next_block_idx != map.rows[i].len() {
                    if j == map.rows[i][next_block_idx] {
                        free_space.set(i, j, 0).unwrap();
                        next_block_idx += 1;
                        continue
                    }
                }
                
                if next_block_idx == 0 { // Before first blocked square in column
                    val = j + 1;
                } else {
                    val = j - map.rows[i][next_block_idx-1];
                }

                free_space.set(i, j, val).unwrap();
            }
        }
        free_space
    }

    pub fn setup_free_space(map: &Map) -> Matrix<(usize,usize,usize,usize)> {

        let up = Path::get_up_free_space(map);    
        let right = Path::get_right_free_space(map);
        let down = Path::get_down_free_space(map);
        let left = Path::get_left_free_space(map);

        let mut free_space = Matrix::new(
            map.height,
            map.width,
            (
                map.width,
                map.height,
                map.width,
                map.height,
            )
        );

        for i in 0..map.rows.len() {
            for j in 0..map.cols.len() {
                free_space.set(i, j, (
                        *up.get(i,j).unwrap(),
                        *right.get(i,j).unwrap(),
                        *down.get(i,j).unwrap(),
                        *left.get(i,j).unwrap())).unwrap();
            }
        }
        free_space
    }

    pub fn parse_input(input: &str) -> Self {
        Path::new(Map::parse_input(input))
    }

    pub fn walk_the_path(&mut self) {
        while self.map.is_inside(self.current_position) {
            println!("Current position, orientation, is_inside: {:?}, {:?}, {}",
                     self.current_position, self.orientation, self.map.is_inside(self.current_position));
            self.walk();
            self.orientation.next();
        }
    }

    pub fn count_visited_positions(&self) -> usize {
        self.visited.iter().filter(|&&v| v).count()
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let mut path: Path = Path::parse_input(&content);
    path.walk_the_path();

    let number_of_visited_positions = path.count_visited_positions();
    println!("Number of visited positions: {}", number_of_visited_positions);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_input() -> &'static str {
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    fn construct_visited_input() -> &'static str {
        "\
....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."
    }

    fn construct_visited(size: usize) -> Matrix<bool> {
        let input = construct_visited_input();
        let mut visited: Matrix<bool> = Matrix::new(size, size, false);
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == VISITED {
                    visited.set(row, col, true).unwrap();
                }
            }
        }
        visited
    }

    fn create_map() -> Map {
        let map: Map = Map::new(
            vec![
                vec![4],
                vec![9],
                vec![],
                vec![2],
                vec![7],
                vec![],
                vec![1],
                vec![8],
                vec![0],
                vec![6],
            ],
            vec![
                vec![8],
                vec![6],
                vec![3],
                vec![],
                vec![0],
                vec![],
                vec![9],
                vec![4],
                vec![7],
                vec![1],
            ],
            (6,4),
        );
        map
    }

    fn create_path() -> Path {
        Path::new(create_map())
    }

    #[test]
    fn test_map_parse_input() {
        let input = construct_input();

        let map = Map::parse_input(input);

        let actual = create_map();
        println!("{:?}", map);
        assert!(map == actual);
    }

    #[test]
    fn test_path_parse_input() {
        let input = construct_input();

        let path = Path::parse_input(input);

        let actual = create_path();
        println!("{:?}", path);
        assert!(path == actual);
    }

    #[test]
    fn test_walk_the_path() {
        let input = construct_input();
        let mut path = Path::parse_input(input);

        let actual: Matrix<bool> = construct_visited(10);

        path.walk_the_path();

        assert!(actual == path.visited);

        assert_eq!(41, path.count_visited_positions());


    }
    #[test]
    fn test_setup_free_space() {
        let map = create_map();

        let free_space = Path::setup_free_space(&map);
        println!("{:?}", free_space);

        assert!(false);
    }

}
