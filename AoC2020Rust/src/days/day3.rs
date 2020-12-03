
pub fn day3() {
    // Part one
    let part_one_count: usize = treesOnSlope(3, 1);
    println!("There are {} trees in part one.", part_one_count);

    // Part two
    let multiplied_slopes: usize = (
        treesOnSlope(1, 1) *
        part_one_count *
        treesOnSlope(5, 1) *
        treesOnSlope(7, 1) *
        treesOnSlope(1, 2)
    );
    println!("Part two's slope counts multiplied is {}", multiplied_slopes);
}

fn treesOnSlope(right: usize, down: usize) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let width = 31;
    let height = data.len();
    let mut num_of_trees: usize = 0;
    let mut reached_bottom: bool = false;
    while !reached_bottom {
        if y >= height {
            reached_bottom = true;
            break;
        }
        if data[y].chars().nth(x % width).unwrap() == '#' {
            num_of_trees += 1;
        }
        y += down;
        x += right;
    }
    num_of_trees
}

const data: [&str; 323] = [
"...............#.#.............",
"##..#....................#...##",
"......#..#.#.....#..#.#.##.....",
".........#...#..............#.#",
"............#.......##.........",
"...#.....#.....#...#.....#..#..",
"..............#..##.#..#......#",
".##.....#.....#......##.#......",
".#..........###....#...##....#.",
".....#....#.#.......#......##..",
".#....#......#.......#........#",
"..#.#.......#..##.....##.......",
"...#.#....#.......#.......#...#",
"##.##...##..#......#.#.....#..#",
".#.#.......#..#.#......#...#.#.",
"#.......##.......#...#.........",
".....#......#.#.#.....#....##..",
".#.#........#....#..#..#.......",
"...#....#..###.........#.....#.",
"........#........#........#....",
"..##..............#.....#.#..#.",
".#...##.............#.#........",
"....#..#...........#.......#...",
"..#....#.....................#.",
"#.#..................##......##",
".#.##....#......#........#.....",
".........##.....#....#...##..#.",
"#..........#..#.#.............#",
".........#...#.#.#.#..##..##...",
"#...#.....#..#..#....#...#.....",
"..##.....#..................#..",
"#..###.....#....#.......#..#...",
"...##.##..#............#......#",
"........###.........###......#.",
"#..##....#.........#.........#.",
"....#.....................#....",
"#..#..##..#..####.##..#.....##.",
"..#...#.#....#....##.....#.....",
"...#.#.........#.....#.#.......",
"....#................#..#...##.",
"....#..#..........#...#.#.##...",
"........#..##............#....#",
"...#......##..........#.##...#.",
".......##......................",
".......##..........#....#.#...#",
"......###.##..##..#....#...#..#",
"#.#...........##.....#........#",
"..#...........#..###....#.#.#..",
"........#...........#......##..",
".........#...##.###...###..#...",
".....#.....#..##.........##....",
"...##..............#.....#...##",
".##....#.......###.....#.......",
".#...........##.............##.",
"......#..#..##.##......#......#",
"........###........#......#.#..",
"#.#....#.....#........#......#.",
".##..#.........##...##....#....",
".....#.........#...##.....#....",
".............#........###....#.",
"......#.......#.#........#.#...",
"..#....#.#...#....#...#.#...##.",
"#...#......##..##......#.##.###",
"...##.#....#...#....#.........#",
"...#..####.....##.#..#.#...##..",
"##.#..#....##......#......##...",
"###.........#.#..#.#.....#.....",
"...#........#..##...#.#.#..#.#.",
"...###..#.###.#...#............",
"....................###........",
"...........#...........#.......",
"#..............#.#.........###.",
"....................##.....#..#",
"#.#.....#.......#...#..........",
".#...#......#....##...#...#....",
".....#.##..................###.",
".........#.#..#.#......#.......",
".......#.....##..#.##.#........",
"..#..........#.###.....#....#..",
"......#.............#.#........",
"........##....#........#.......",
"...#.............#....#.#......",
"#........#..####.....#.....#.#.",
".##......##...#........#..#.#..",
"....##....#...#...#..##...#.#..",
"#.##...###..#....##.#..........",
"....#.#...#.#...#..##.###...#..",
"#.....##..#..#....#.#.....##...",
".#..#..........##.#.....##.....",
".#..#........#.#.#.#...........",
".#..#.....#...........#...#....",
"...#......##..........##..#....",
"...#..#....#.##...#..#.....###.",
"#.#....#.....##................",
"#..#......#.#.#.......#........",
"......#....#.#....#..##....#..#",
".#.....#.#....###.##.........#.",
".###..#.....#........#.#.......",
".#...#......#..#.#......#.....#",
"#...............####...#.....#.",
".......#..........##.#........#",
"#........##....##.....###..##..",
"#..#.....#..##.....#....#..#...",
"#.....#.......##......#.#.....#",
"#.##..#......##..#.............",
"##...#.....#........##.........",
"....#..##....#...#.......#.#...",
"....#...#...##..#....#..#...#..",
"..............#.#...#....###...",
"...#....#..##...##..#....##....",
"#.##.#..#..#......#.#.#.#...#..",
".......#..#..##........#......#",
"##.#....#....##.#......##.#....",
".#...#..............#........#.",
".#.#....#.........#............",
".#..#..###.............#....#..",
"#......#...#.#..##..#...#....#.",
".......................#...#.#.",
".............#..#...##.........",
"..#.#..#....#....#........#....",
"#......#.##..#...#.#...........",
".....#....#...........##.#..#..",
"..#.#.....#..............#.#...",
"#.......#.....#................",
"#..............#...#....#...#..",
"...#...##..#..#............#...",
"......###.....................#",
".........#.......##..#....#....",
"........#...#.##..#.##......#..",
"....###..#.#...#...#..#.#...###",
"##...#...##.#...#.#...#.#....#.",
".........#...#.....###.........",
"...#........##..#.......##.....",
".#.......##.........#.....##..#",
".#..................#...#......",
".##..#..#.#.....#.###..........",
"...#.....##..#.........#...#...",
".#......#.#.......#.#..........",
".........#.#...#..........#.#..",
"#..........#.##..#.##....#.....",
".#.#....#.....#..##.....#...#..",
"..#........##...##..#..#....#..",
"#...........##....#..###....#..",
"...........##.........####...#.",
"..#........###...#.#.........#.",
".#...............#.##.#.#...#..",
".#.##..#.....#.#.....##..#.....",
"...#...#..#.##.##...#.......##.",
"..#...#...#......##.##.##...#..",
"##....#...#...#...............#",
"...##...........#......#..#.#..",
"#.........#......#.#.##.....#..",
"........#..#.........##........",
"..#.#....###.....##..#...#.....",
".........#...#.......#.....##..",
"##.....................#...##..",
".#.#..#......#.................",
".....###..#......#..###..#.....",
"...#.....##.........#......#..#",
"......##.....#...#........#.#..",
"..#.#...#......#...#.##.##.....",
"...#..........#...#.......#..##",
".###........#........##........",
"..#.#.#..........#.#...##......",
".........#........#......###..#",
"....##..#.........#...........#",
"..####..#............##.......#",
".....##.#..##.........#...#.#..",
"...#.........#.....#.....#.....",
".......#...#..#...##.........#.",
"...#...#..#...#....#..#........",
"#............##.##...#.........",
".#.#.....#.......####.....#....",
"..............#......#.#.......",
"..............#...........#...#",
"#...#........###....#.#....#.#.",
"##.#..#..#......#......#.#.#...",
".#..#.....#..#.#..#.#.......##.",
"......##.#...#...#......#...#..",
"#...........##....#.#..........",
"....#.......###.#...#..........",
".......................#.....#.",
"........#...#..#...#.#.#.#.#...",
".#.#...........#......##...#...",
".........................#.....",
".................#.##.#...##...",
"...#...##.....#.....##....#.#..",
"...#...#...................#...",
"...#..#..#...#...#....#........",
"#....#...#.....#...............",
".......#...........#...#.......",
"....#....#.....##.......#......",
".......#..........##...........",
".#.#........#..##....#......#..",
".....#.......#.#.........#...#.",
".#..####.#.#...............#..#",
".....###..#..#..........#.#..##",
"..#.......#...#.....##..#..#.#.",
"#....#......#..................",
"........#.##.#....#...........#",
"....#.#....##..#.#.....##......",
"...#..#.......#....#.....#.#.#.",
"#...#......#.....#.#..........#",
"....#....#...............#.....",
"..###......................###.",
".##....#..#.......###.....#..#.",
"..###............#........#.##.",
".#........#......#.....#..#....",
"....#..##...#...#.###.......#.#",
".......#.##...........#.#..#...",
".....#...##....................",
"....#....#...##......#.........",
"..#............##....###.#...#.",
".#........#...............#....",
"#..#.#.##.........#..##....##..",
"#.#....#..#.##....##...#.#.....",
".....#.....##....#.#........#..",
"#..#...#...#....#....#.........",
"...#........#..#.#.....##......",
"..#...#...#................##..",
"#........#.#.##.......#.#...#..",
"#......#..####.##.....#.#..#.#.",
"............#..#.#....#......##",
"..#.....##....#...#.#..........",
"...#...#.........#...#.#.......",
".###..#.......##.##.....#.#.#..",
"...#....#...............##.#...",
"....##..#..#..#.#......##.....#",
"#.#..............##...##...####",
".....#.##...#.#...............#",
".##.....#.........#.......#.#.#",
"#.#..#.....#.......#.......#..#",
"...#.#.....#.....#......#......",
".......#....#..#.#..........#..",
"......#......#.##...#..........",
".....#.......###...#...#.#.....",
"#..#.#.........#.....#.##....#.",
"..#.#.........#..#..#..#.....#.",
".#..##..#..#....#......#.##..#.",
"...##......###.....#.##.##.....",
".#.....#...#..#...#............",
"##..##..#.##....#..#...........",
"...#..##..#..#.............#.##",
"...............##............#.",
"..#.....##........##.#...#....#",
".#.#...#.#.#..#.#.....#....#...",
".#....#...............#..#.....",
"....#.##..#....#......#...###..",
"#................###...#.#.....",
"...#...#......##..#.#....#.....",
".#....#....#.#...##............",
"....#...##..#..#........#.##...",
"..##.....#..#..##..............",
"..#..##..#.#..##....#....#....#",
"...##.............#............",
"#....#....#.#........#.....##.#",
".....#..#.#.....####...###.....",
"................#......#.......",
".....#.#.#.#.#....#..#........#",
".##.#...#.#.......##....#....#.",
".....#........#................",
"..#.....#..#...#..#...........#",
".#.....#...##.....##..#.#....##",
"......#.......#..#......##.#...",
"#.#..........#.##.#........#...",
"...#..#.............#..........",
"#..#..#..........#..##.#.......",
".#..#...............####..#....",
".......#.....#......#.....#.#..",
".#...............#...#.........",
".#..#..........#..#.#..##..#..#",
"......##..#.....#..#......###..",
"..........#...#..#.......#.....",
".#.#.#..#.....#.##.#...#..#....",
"........#.......#.....#.#......",
"......#.....##.....#....##.#...",
"...............#......#.......#",
"..#.#...#.....#.#...##......#..",
"#.#.........#.#...#........####",
"#..........##..#..#........##..",
".............#..#.......##.#..#",
"..#........#.#....#........#.#.",
".#......####..#..#.............",
"............###.......#.#..#...",
"#.##......##...#...#.........#.",
"....##.#.#.#......#....#..#...#",
".#..#.#....#...#.........#.....",
"#...#.....##............#...#..",
"#.#...#..#.................#...",
"............#.#..#.....#.#.#..#",
"...................#....#.##...",
".....#...#.#....#....#.#......#",
".......##.#.#......##..........",
".#..#...##.#...#..#......#.....",
"......#.#..#..###..##..##......",
".#.#.#.#.....#...###.....#..#..",
".#....#.....#.......#.......#..",
"..........##.........####......",
".#.#.............#..#.#...#....",
"........#........##...#.#....#.",
"........#......................",
"..#.#....#...............#...##",
".......#.#...#..#.....##......#",
".#...#....#..........##........",
".#.........#.#............##...",
".....#......##...#.......#..#..",
"#.#..#.............#...#...#...",
"......#.......#............#...",
"...........##....#......##.....",
".#.#..#.....................#..",
"##..##.....###..##.#...........",
"...##......##....#...##.....#..",
"#...#.##.............#.........",
"......#..#.........###.#......#",
"#.#.....#.....................#",
"....#####.....##........#.#..#.",
"...........##..##.###..........",
"..........##.....#........#...#",
".......#..#......#.....##..##.#",
".....##.#........#.........#...",
"......##......................#",
".#.......#.#.#............#..#.",
".....##.#.......#.#........#..."];