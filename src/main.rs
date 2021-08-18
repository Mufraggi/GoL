fn main() {
    println!("Hello, world!");
    init_map(3);
}

fn init_map(size:u32) -> Vec<Vec<bool>>{
    let mut map:Vec<Vec<bool>> =  vec![];
    for  _ in 0..size {
        let mut row:Vec<bool> =vec![];
        for _ in 0..size {
            row.push(false);
        }
        map.push(row)
    }
    map
}

fn change_value(mut map:Vec<Vec<bool>>, x:u32, y:u32, value:bool) -> Vec<Vec<bool>> {
    map[y as usize][x as usize] = value;
    map
}

fn count_neibor(map:Vec<Vec<bool>>, x:i32, y:i32) -> i32 {
    let possibilitys: [(i32, i32); 8] = [
        (x - 1,y - 1), ( x, y - 1),( x + 1 ,y - 1),
        (x - 1,y), (x+1,y),
        (x -1 ,y + 1), (x, y + 1),( x + 1,y + 1),];


    let mut count: i32 = 0;
    let map_size:usize = map.len();

    for possibility in possibilitys.iter() {
        count = is_good(map_size, possibility.0, possibility.1, count as u32, map.clone()) as i32;
    }
    count
}

fn is_good(map_len:usize,value_x:i32, value_y:i32, mut count:u32, map:Vec<Vec<bool>>) -> u32 {
    if value_x >= 0 && value_y >= 0 && value_x < map_len as i32 &&value_y < map_len as i32 {
        if map[value_y as usize][value_x as usize] == true {
            count = count + 1;
        }
    }
    count
}

fn generate(map:Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new_map = init_map(map.len() as u32);
    for y in 0..map.len() {
        for x in 0..map.len() {
            let nb_neibor = count_neibor(map.clone(), x as i32, y as i32);
            new_map = rules(nb_neibor as u32, new_map, x as u32, y as u32, map[y][x]);
        }
    }
    new_map
}

fn rules(nb_count:u32, mut new_map:Vec<Vec<bool>>, x:u32, y:u32, cel_value:bool) -> Vec<Vec<bool>> {
    if nb_count == 2 {
        new_map = change_value(new_map,x, y,cel_value);
        return new_map
    }
    if nb_count < 2 || nb_count > 3 {
        new_map = change_value(new_map,x, y,false);
        return new_map
    }
    if nb_count == 3 {
        new_map = change_value(new_map,x, y,true);
        return new_map
    }
    return new_map
}

#[cfg(test)]
mod tests {
    use crate::{init_map, generate};
    use crate::change_value;
    use crate::count_neibor;

    #[test]
    fn init_map_test() {
        let map = init_map(3);
        for  x in 0..3 {
            for y in 0..3 {
                assert_eq!(map[x][y], false)
            }
        }
    }
    #[test]
    fn set_value_test() {
        let mut  map = init_map(3);
        map = change_value(map, 1,1, true);
        assert_eq!(map,[
            [false,false,false],
            [false,true,false],
            [false,false,false]
        ])
    }
    #[test]
    fn count_neibor_test() {
        let mut  map = init_map(3);
        map = change_value(map, 1,0, true);
        map = change_value(map, 1,1, true);
        map = change_value(map, 1,2, true);
        assert_eq!(map,[
            [false,true,false],
            [false,true,false],
            [false,true,false]
        ]);
        assert_eq!(count_neibor(map, 1, 1), 2)
    }
    #[test]
    fn rule_underpop() {
        let mut  map = init_map(3);
        map = change_value(map, 0,1, true);
        map = change_value(map, 2,1, true);
        assert_eq!(map,[
            [false, false, false],
            [true, false, true],
            [false, false, false]
        ]);
        map = generate(map);
        assert_eq!(map,  [[false, false, false],
                   [false, false, false],
                   [false, false, false]])
    }
    #[test]
    fn rule_underpop2() {
        let mut  map = init_map(3);
        map = change_value(map, 0,1, true);
        map = change_value(map, 1,1, true);
        map = change_value(map, 2,1, true);
        assert_eq!(map,[
            [false, false, false],
            [true, true, true],
            [false, false, false]
        ]);
        map = generate(map);
        assert_eq!(map, [[false, true, false],
            [false, true, false],
            [false, true, false]])
    }
     #[test]
    fn rule_born() {
        let mut  map = init_map(3);
        map = change_value(map, 0,1, true);
        map = change_value(map, 1,1, true);
        map = change_value(map, 2,1, true);
        assert_eq!(map,[
            [false, false, false],
            [true, true, true],
            [false, false, false]
        ]);
        map = generate(map);
        assert_eq!(map, [[false, true, false],
            [false, true, false],
            [false, true, false]])
    }

    #[test]
    fn rule_surpop() {
        let mut  map = init_map(3);
        map = change_value(map, 0,1, true);
        map = change_value(map, 1,1, true);
        map = change_value(map, 2,1, true);
        map = change_value(map, 1,0, true);
        assert_eq!(map,[
            [false, true, false],
            [true, true, true],
            [false, false, false]
        ]);
        map = generate(map);
        assert_eq!(map, [[true, true, true],
            [true, true, true],
            [false, true, false]]);
        map = generate(map);
        assert_eq!(map, [[true, false, true],
            [false, false, false],
            [true, true, true]]);
    }
}