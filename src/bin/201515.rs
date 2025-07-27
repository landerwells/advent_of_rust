fn main() {
    // let input = parse(get_input(2015, 15));
    // println!("{}", input);

    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
}

// fn parse(input: String) -> String {
//     input
// }

fn part_one() -> i32 {
    solve(false)
}

fn part_two() -> i32 {
    solve(true)
}

fn solve(part_two: bool) -> i32 {
    let t = [
        [5, -1, 0, 0, 5],
        [-1, 3, 0, 0, 1],
        [0, -1, 4, 0, 6],
        [-1, 0, 0, 2, 8]
    ];

    let mut max = 0;
    
    for i in 0..100 {
        for j in 0..100-i {
            for k in 0..100-i-j {
                let h = 100-i-j-k;
                let a = t[0][0]*i+t[1][0]*j+t[2][0]*k+t[3][0]*h;
                let b = t[0][1]*i+t[1][1]*j+t[2][1]*k+t[3][1]*h;
                let c = t[0][2]*i+t[1][2]*j+t[2][2]*k+t[3][2]*h;
                let d = t[0][3]*i+t[1][3]*j+t[2][3]*k+t[3][3]*h;
                let e = t[0][4]*i+t[1][4]*j+t[2][4]*k+t[3][4]*h;


                if part_two && e != 500 {
                    continue;
                }

                if a <= 0 || b <= 0 || c <= 0 || d <= 0 {
                    continue;
                }

                let score = a*b*c*d;
                if score > max {
                    max = score;
                }

            }

        }

    }
    
    max
} 

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_part_one() {}
//
//     #[test]
//     fn test_part_two() {}
// }
