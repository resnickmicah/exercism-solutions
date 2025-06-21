use self::TileType::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum TileType {
    Corner,
    Horizontal,
    Vertical,
    Space,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
    tile_type: TileType,
}

#[derive(PartialEq, Eq, Hash)]
pub struct Side {
    begin: Point,
    end: Point,
}

fn populate_points(rows: &mut Vec<Vec<Point>>, lines: &[&str]) {
    for (y, line) in lines.iter().enumerate() {
        let mut columns: Vec<Point> = vec![];
        for (x, rchar) in line.chars().enumerate() {
            columns.push(Point {
                x,
                y,
                tile_type: match rchar {
                    '+' => Corner,
                    '-' => Horizontal,
                    '|' => Vertical,
                    ' ' => Space,
                    _ => panic!("Invalid tile type"),
                },
            });
        }
        rows.push(columns);
    }
}

fn populate_sides(sides: &mut HashSet<Side>, points: &Vec<Vec<Point>>) {
    for row in points {
        // horizontal sides
        for point in row {
            if point.tile_type == Corner {
                let next_right = point.x + 1;
                scan_right(sides, &point, &row[next_right..]);
                let column = make_column(&point, &points);
                scan_down(sides, &point, column);
            }
        }
    }
    ()
}

fn scan_right(sides: &mut HashSet<Side>, starting_point: &Point, row: &[Point]) {
    use self::TileType::*;
    for right_point in row {
        match right_point.tile_type {
            Corner => sides.insert(Side {
                begin: starting_point.clone(),
                end: right_point.clone(),
            }),
            Horizontal => continue,
            _ => break,
        };
    }
}

fn make_column<'a>(starting_point: &Point, grid: &'a Vec<Vec<Point>>) -> Vec<&'a Point> {
    grid.into_iter()
        .flatten()
        .filter(|p| p.x == starting_point.x && p.y > starting_point.y)
        .collect()
}

fn scan_down(sides: &mut HashSet<Side>, starting_point: &Point, column: Vec<&Point>) {
    for down_point in column {
        match down_point.tile_type {
            Corner => sides.insert(Side {
                begin: starting_point.clone(),
                end: down_point.clone(),
            }),
            Vertical => continue,
            _ => break,
        };
    }
}

fn count_rectangles(grid: &Vec<Vec<Point>>, sides: &HashSet<Side>) -> u32 {
    let top_left_corners = all_corners(grid);
    let mut rectangle_count: u32 = 0;

    for top_left in top_left_corners {
        let maybe_br = diagonals_from(top_left, grid);
        for bottom_right in maybe_br {
            if check_sides(top_left, bottom_right, sides) {
                rectangle_count += 1;
            }
        }
    }
    return rectangle_count;
}

fn all_corners(grid: &Vec<Vec<Point>>) -> Vec<&Point> {
    grid.into_iter()
        .flatten()
        .filter(|p| p.tile_type == Corner)
        .collect()
}

fn diagonals_from<'a>(starting_point: &Point, other_points: &'a Vec<Vec<Point>>) -> Vec<&'a Point> {
    other_points
        .into_iter()
        .flatten()
        .filter(|p| p.tile_type == Corner && p.x > starting_point.x && p.y > starting_point.y)
        .collect()
}

fn check_sides(top_left: &Point, bottom_right: &Point, sides: &HashSet<Side>) -> bool {
    let tl = top_left.clone();
    let tr = Point {
        x: bottom_right.x,
        y: top_left.y,
        tile_type: Corner,
    };
    let bl = Point {
        x: top_left.x,
        y: bottom_right.y,
        tile_type: Corner,
    };
    let br = bottom_right.clone();
    let top = Side { begin: tl, end: tr };
    let left = Side { begin: tl, end: bl };
    let right = Side { begin: tr, end: br };
    let bottom = Side { begin: bl, end: br };
    sides.contains(&top)
        && sides.contains(&bottom)
        && sides.contains(&left)
        && sides.contains(&right)
}

pub fn count(lines: &[&str]) -> u32 {
    if lines.len() == 0 {
        return 0;
    }

    let mut points: Vec<Vec<Point>> = vec![];
    let mut sides: HashSet<Side> = HashSet::new();

    populate_points(&mut points, lines);
    // enumerate rectangle sides
    populate_sides(&mut sides, &points);
    // construct rectangles from pairs of points on different rank+files
    count_rectangles(&points, &sides)
}
