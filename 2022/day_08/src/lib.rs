use std::collections::HashSet;

fn parse_tree_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|ch| ch.to_digit(10))
                .map(|size| size + 1)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

fn record_tree_visibility(grid: Vec<Vec<u32>>) -> HashSet<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visibility = HashSet::new();

    for (i, row) in grid.iter().enumerate().take(height) {
        let mut l_to_r_max = 0;
        let mut r_to_l_max = 0;
        for j in 0..width {
            let left = row[j];
            let right = row[width - j - 1];
            if left > l_to_r_max {
                l_to_r_max = left;
                visibility.insert((i, j));
            }
            if right > r_to_l_max {
                r_to_l_max = right;
                visibility.insert((i, width - j - 1));
            }
        }
    }

    for j in 0..width {
        let mut t_to_b_max = 0;
        let mut b_to_t_max = 0;
        for i in 0..height {
            let top = grid[i][j];
            let bottom = grid[height - i - 1][j];
            if top > t_to_b_max {
                t_to_b_max = top;
                visibility.insert((i, j));
            }
            if bottom > b_to_t_max {
                b_to_t_max = bottom;
                visibility.insert((height - i - 1, j));
            }
        }
    }

    visibility
}

fn find_longest_sightline(grid: Vec<Vec<u32>>) -> u32 {
    let mut longest_sightline_score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, tree_height) in row.iter().enumerate() {
            let mut views = (0, 0, 0, 0);
            for ii in (0..i).rev() {
                    views.0 += 1;
                if grid[ii][j] >= *tree_height {
                    break;
                }
            }
            for row in grid.iter().skip(i+1) {
                    views.1 += 1;
                if row[j] >= *tree_height {
                    break;
                }
            }
            for jj in (0..j).rev() {
                    views.2 += 1;
                if grid[i][jj] >= *tree_height {
                    break;
                }
            }
            for jj in (j + 1)..grid[i].len() {
                    views.3 += 1;
                if grid[i][jj] >= *tree_height {
                    break;
                }
            }

            let score = views.0 * views.1 * views.2 * views.3;
            longest_sightline_score = longest_sightline_score.max(score);
        }
    }
    longest_sightline_score
}

pub fn part_1(input: &str) -> String {
    let grid = parse_tree_grid(input);
    let visibility = record_tree_visibility(grid);
    visibility.len().to_string()
}

pub fn part_2(input: &str) -> String {
    let grid = parse_tree_grid(input);
    let best_score = find_longest_sightline(grid);
    best_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "30373\n",
        "25512\n",
        "65332\n",
        "33549\n",
        "35390\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "21");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "8");
    }
}
