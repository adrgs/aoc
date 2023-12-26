use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

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
            return 999999;
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

    let ans = dfs(&matrix, 1, 1, &mut visited) - 999999 + 2;

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut graph: HashMap<(usize, usize), HashMap<(usize, usize), i32>> = HashMap::new();

    let mut matrix: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    matrix[0][1] = '#';
    let (m, n) = (matrix.len(), matrix[0].len());

    matrix[m - 1][n - 2] = '#';

    let mut nodes: Vec<(usize, usize)> = Vec::new();

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            if matrix[i][j] != '#' {
                let neighs = (matrix[i - 1][j] != '#') as i32 + (matrix[i + 1][j] != '#') as i32
                    + (matrix[i][j - 1] != '#') as i32
                    + (matrix[i][j + 1] != '#') as i32;

                if neighs != 2 {
                    nodes.push((i, j));
                }
            }
        }
    }

    fn traverse(matrix: &Vec<Vec<char>>, start: (usize, usize), i: usize, j: usize) -> Option<((usize, usize), i32)> {
        let mut dist = 1;
        let mut i = i;
        let mut j = j;

        let mut history = HashSet::new();
        history.insert(start);
        history.insert((i, j));

        if matrix[i][j] == '#' {
            return None;
        }

        loop {
            let neighbors = vec![
                (i - 1, j),
                (i + 1, j),
                (i, j - 1),
                (i, j + 1),
            ];

            for (ii, jj) in neighbors {
                if history.contains(&(ii, jj)) {
                    continue;
                }
                if matrix[ii][jj] == '#' {
                    continue;
                }

                let neighs = (matrix[ii - 1][jj] != '#') as i32 + (matrix[ii + 1][jj] != '#') as i32
                    + (matrix[ii][jj - 1] != '#') as i32
                    + (matrix[ii][jj + 1] != '#') as i32;

                if neighs != 2 {
                    return Some(((ii, jj), dist + 1));
                }

                i = ii;
                j = jj;
                break;
            }

            history.insert((i, j));

            dist += 1;
        }
    }

    for point in nodes {
        for direction in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let j = point.1 as i32 + direction.1;
            let i = point.0 as i32 + direction.0;

            let opt = traverse(&matrix, point, i as usize, j as usize);

            if opt.is_none() {
                continue;
            }
            let (neigh, dist) = opt.unwrap();

            graph.entry(point).or_insert(HashMap::new()).insert(neigh, dist);
            graph.entry(neigh).or_insert(HashMap::new()).insert(point, dist);
        }
    }

    fn dfs(
        graph: &HashMap<(usize, usize), HashMap<(usize, usize), i32>>,
        start: (usize, usize),
        end: (usize, usize),
        visited: &mut HashSet<(usize, usize)>
    ) -> i32 {
        if visited.contains(&start) {
            return 0;
        }

        if start == end {
            return 999999;
        }

        visited.insert(start);

        let mut ans = 0;

        for (neigh, d) in graph.get(&start).unwrap() {
            ans = ans.max(d + dfs(graph, *neigh, end, visited));
        }

        visited.remove(&start);

        ans
    }

    let ans = dfs(&graph, (1, 1), (m-2, n-2), &mut HashSet::new()) - 999999 + 2;

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
