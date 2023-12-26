use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    matrix[0][1] = '#';
    let (m, n) = (matrix.len(), matrix[0].len());

    matrix[m - 1][n - 2] = '#';

    fn dfs(matrix: &Vec<Vec<char>>, i: usize, j: usize, visited: &mut Vec<Vec<bool>>) -> i32 {
        if i == matrix.len() - 2 && j == matrix[0].len() - 2 {
            return 1;
        }

        if matrix[i][j] == '#' || visited[i][j] {
            return 0;
        }

        visited[i][j] = true;

        let mut ans = 1;

        match matrix[i][j] {
            '^' => ans = ans + dfs(matrix, i - 1, j, visited),
            'v' => ans = ans + dfs(matrix, i + 1, j, visited),
            '>' => ans = ans + dfs(matrix, i, j + 1, visited),
            '<' => ans = ans + dfs(matrix, i, j - 1, visited),
            _ => {
                ans = ans
                    + dfs(matrix, i + 1, j, visited).max(
                        dfs(matrix, i - 1, j, visited).max(
                            dfs(matrix, i, j + 1, visited).max(dfs(matrix, i, j - 1, visited)),
                        ),
                    )
            }
        }

        visited[i][j] = false;

        ans
    }

    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];

    let ans = dfs(&matrix, 1, 1, &mut visited) + 1;

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
