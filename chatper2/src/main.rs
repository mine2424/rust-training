// use crate_name::module_name_1::module_name2...
use num_bigint::BigInt;
use rand::Rng;

fn main() {
    // get bigint above u128, i128
    get_bigint();
    println!();

    // get random number
    dice();
    println!();

    // get range
    refer_range();
    println!();

    // get maze
    maze();

    println!("---------------- done -----------------");
}

fn get_bigint() {
    // under code create error -> thread 'main' panicked at 'attempt to multiply with overflow'
    // println!("v = {}", 1234_i32.pow(5678));

    // use num_bigint::BigInt;
    let v = BigInt::from(1234);
    println!("v = {}", v.pow(5));
}

fn dice() {
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}

fn refer_range() {
    let range = 1..10;
    println!("{}, {}", range.start, range.end);
}

fn maze() {
    const MAZE_SIZE: usize = 25;

    let mut rng = rand::thread_rng();
    // 迷路を全部通路として初期化する
    let mut maze = [[0; MAZE_SIZE]; MAZE_SIZE];
    // 迷路の外周を壁とする
    for n in 0..MAZE_SIZE {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAZE_SIZE - 1] = 1;
        maze[MAZE_SIZE - 1][n] = 1;
    }
    // 2マスに1つ壁を設置する
    for y in 2..MAZE_SIZE - 2 {
        for x in 2..MAZE_SIZE - 2 {
            if x % 2 == 0 || y % 2 == 1 {
                continue;
            }
            maze[y][x] = 1;
            // 上記で壁を配置した場所ごとにランダムに上下左右いずれかの方向に壁を設置する
            let r = rng.gen_range(0..=3);
            // 迷路全体に１マスおきに壁を設置する
            match r {
                // match_value(to r) => { ... }
                0 => maze[y - 1][x] = 1, // 上
                1 => maze[y + 1][x] = 1, // 下
                2 => maze[y][x - 1] = 1, // 左
                3 => maze[y][x + 1] = 1, // 右
                _ => unreachable!(),
            }
        }
    }
    let tiles = [" ", "ZZ"];
    for y in 0..MAZE_SIZE {
        for x in 0..MAZE_SIZE {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
