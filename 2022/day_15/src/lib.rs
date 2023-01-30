use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list0,
    IResult,
};

#[derive(Debug)]
struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, sensor_x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, sensor_y) = complete::i32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, beacon_x) = complete::i32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, beacon_y) = complete::i32(input)?;
    Ok((
        input,
        Sensor {
            position: (sensor_x, sensor_y),
            beacon: (beacon_x, beacon_y),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list0(newline, parse_sensor)(input)
}

fn get_blocked_in_row(sensors: Vec<Sensor>, row_index: i32) -> HashSet<i32> {
    sensors.iter().fold(
        HashSet::new(),
        |mut row,
         Sensor {
             position: (position_x, position_y),
             beacon: (beacon_x, beacon_y),
         }| {
            let manhattan_distance =
                get_manhattan_distance((*position_x, *position_y), (*beacon_x, *beacon_y));
            let offset_from_row = position_y.max(&row_index) - position_y.min(&row_index);
            let blocked_radius = manhattan_distance - offset_from_row;
            for x in (position_x - blocked_radius)..(position_x + blocked_radius) {
                row.insert(x);
            }
            row
        },
    )
}

fn search_for_gap(sensors: Vec<Sensor>, max_coord: i32) -> (i32, i32) {
    let sensors_with_distance: Vec<(&Sensor, i32)> = sensors
        .iter()
        .map(|sensor| {
            (
                sensor,
                get_manhattan_distance(sensor.position, sensor.beacon),
            )
        })
        .collect();

    for y in 0..=max_coord {
        let mut x = 0;
        while x <= max_coord {
            if let Some(new_x) = sensors_with_distance.iter().find_map(
                |(
                    Sensor {
                        position: (position_x, position_y),
                        ..
                    },
                    manhattan_distance,
                )| {
                    let offset_from_row = (position_y - y).abs();
                    let blocked_radius = manhattan_distance - offset_from_row;
                    if blocked_radius < 0 {
                        return None;
                    }
                    if (x - position_x).abs() > blocked_radius {
                        return None;
                    }
                    Some(position_x + blocked_radius + 1)
                },
            ) {
                x = new_x;
            } else {
                return (x, y);
            }
        }
    }
    panic!()
}

fn get_manhattan_distance(
    (position_x, position_y): (i32, i32),
    (beacon_x, beacon_y): (i32, i32),
) -> i32 {
    let x_distance = position_x.max(beacon_x) - position_x.min(beacon_x);
    let y_distance = position_y.max(beacon_y) - position_y.min(beacon_y);
    x_distance + y_distance
}

pub fn part_1(input: &str, row_index: i32) -> String {
    let (_, sensors) = parse_input(input).unwrap();
    let blocked_in_row = get_blocked_in_row(sensors, row_index);
    blocked_in_row.len().to_string()
}

pub fn part_2(input: &str, max_coord: i32) -> String {
    let (_, sensors) = parse_input(input).unwrap();
    let (x, y) = search_for_gap(sensors, max_coord);
    (x as u64 * 4_000_000 + y as u64).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const INPUT: &str = concat!(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16\n",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3\n",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16\n",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16\n",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16\n",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10\n",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10\n",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10\n",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17\n",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22\n",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3\n",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3\n",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3\n",
    );

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(INPUT, 10), "26");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(INPUT, 20), "56000011");
    }
}
