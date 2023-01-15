use nom::{
    character::{
        complete::{newline, satisfy},
        is_alphabetic,
    },
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug)]
enum TileType {
    Start,
    End,
    Path,
}

#[derive(Debug)]
struct Tile {
    tile_type: TileType,
    distance: u32,
    elevation: u32,
}

fn parse_row(input: &str) -> IResult<&str, Vec<Tile>> {
    let (input, row) = many1(satisfy(|ch| is_alphabetic(ch as u8)))(input)?;
    let row = row
        .iter()
        .map(|&tile| Tile {
            tile_type: match tile {
                'S' => TileType::Start,
                'E' => TileType::End,
                _ => TileType::Path,
            },
            elevation: match tile {
                'S' => 'a' as u32,
                'E' => 'z' as u32,
                elevation => elevation as u32,
            },
            distance: u32::MAX,
        })
        .collect();
    Ok((input, row))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    let (input, grid) = separated_list1(newline, parse_row)(input)?;
    Ok((input, grid))
}

fn walk_grid(grid: &mut Vec<Vec<Tile>>, end: (usize, usize)) {
    let mut stack = vec![end];
    grid[end.0][end.1].distance = 0;
    while let Some((i, j)) = stack.pop() {
        let current_distance = grid[i][j].distance;
        let current_elevation = grid[i][j].elevation;

        if i > 0 {
            let mut up = &mut grid[i - 1][j];
            if up.elevation >= current_elevation - 1 && up.distance > current_distance + 1 {
                up.distance = current_distance + 1;
                stack.push((i - 1, j));
            }
        }

        if i < grid.len() - 1 {
            let mut down = &mut grid[i + 1][j];
            if down.elevation >= current_elevation - 1 && down.distance > current_distance + 1 {
                down.distance = current_distance + 1;
                stack.push((i + 1, j));
            }
        }

        if j > 0 {
            let mut left = &mut grid[i][j - 1];
            if left.elevation >= current_elevation - 1 && left.distance > current_distance + 1 {
                left.distance = current_distance + 1;
                stack.push((i, j - 1));
            }
        }

        if j < grid[0].len() - 1 {
            let mut right = &mut grid[i][j + 1];
            if right.elevation >= current_elevation - 1 && right.distance > current_distance + 1 {
                right.distance = current_distance + 1;
                stack.push((i, j + 1));
            }
        }
    }
}

pub fn part_1(input: &str) -> String {
    let (_, mut grid) = parse_input(input).unwrap();

    let mut end = None;
    let mut start = None;
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let TileType::Start = tile.tile_type {
                start = Some((i, j));
            }
            if let TileType::End = tile.tile_type {
                end = Some((i, j));
            }
        }
    }

    walk_grid(&mut grid, end.unwrap());
    grid[start.unwrap().0][start.unwrap().1]
        .distance
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let (_, mut grid) = parse_input(input).unwrap();

    let mut end = None;
    let mut candidates = vec![];
    for (i, row) in grid.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if let TileType::End = tile.tile_type {
                end = Some((i, j));
            }
            if tile.elevation == 'a' as u32 {
                candidates.push((i, j));
            }
        }
    }

    walk_grid(&mut grid, end.unwrap());
    candidates
        .iter()
        .map(|&(i, j)| grid[i][j].distance)
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "Sabqponm\n",
        "abcryxxl\n",
        "accszExk\n",
        "acctuvwj\n",
        "abdefghi\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT), "31");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT), "29");
    }
}
