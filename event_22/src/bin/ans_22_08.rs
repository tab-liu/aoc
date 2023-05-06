use std::fs;
use std::path::PathBuf;

fn part_one(group: &Vec<Vec<u8>>) -> usize {
    let (rows, cols) = (group.len(), group[0].len());
    let mut visible = vec![vec![false; cols]; rows];
    // let mut ans = 2 * (rows + cols) - 4;

    // left -> right
    for (i, rows) in group.iter().enumerate() {
        let mut tallest = rows[0];
        visible[i][0] = true;
        for j in 1..rows.len() {
            if rows[j] > tallest {
                visible[i][j] = true;
                tallest = rows[j];
            }
        }
    }

    // right -> left
    for (i, rows) in group.iter().enumerate() {
        let mut tallest = rows[cols - 1];
        visible[i][cols - 1] = true;
        for j in (0..rows.len() - 1).rev() {
            if rows[j] > tallest {
                visible[i][j] = true;
                tallest = rows[j];
            }
        }
    }

    // top -> bottom
    for j in 0..cols {
        let mut tallest = group[0][j];
        visible[0][j] = true;
        for i in 1..rows {
            if group[i][j] > tallest {
                visible[i][j] = true;
                tallest = group[i][j];
            }
        }
    }

    // bottom -> top
    for j in 0..cols {
        let mut tallest = group[rows - 1][j];
        visible[rows - 1][j] = true;
        for i in (0..rows - 1).rev() {
            if group[i][j] > tallest {
                visible[i][j] = true;
                tallest = group[i][j];
            }
        }
    }

    visible.iter().flat_map(|v| v).filter(|&b| *b).count()
}

fn score(group: &Vec<Vec<u8>>, i: usize, j: usize) -> usize {
    0
}

fn part_two(group: &Vec<Vec<u8>>) -> usize {
    let mut ans = 0;
    for i in 1..group.len() - 1 {
        for j in 1..group[0].len() - 1 {
            ans = ans.max(score(group, i, j));
        }
    }
    ans
}

fn main() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/bin/input_08.txt");

    let mut group = vec![];

    let content = fs::read_to_string(path).unwrap();
    for line in content.lines() {
        group.push(line.chars().map(|c| c as u8 - b'0').collect::<Vec<_>>());
    }

    println!("part_one: {:?}", part_one(&group));
    println!("part_two: {:?}", part_two(&group));
}
