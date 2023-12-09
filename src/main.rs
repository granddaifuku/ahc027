use proconio::{input, marker::Chars};
use std::cmp;

#[allow(non_snake_case)]
#[derive(Debug)]
struct Input {
    N: usize,
    h: Vec<Vec<bool>>,
    v: Vec<Vec<bool>>,
    #[allow(unused)]
    d: Vec<Vec<u32>>,
}

impl Input {
    #[allow(non_snake_case)]
    fn read() -> Self {
        input! {
            N: usize,
            hi: [Chars; N-1],
            vi: [Chars; N],
            d: [[u32; N]; N],
        }

        // True: walkable
        let mut h = vec![vec![true; N]; N - 1];
        let mut v = vec![vec![true; N - 1]; N];
        for i in 0..N - 1 {
            for j in 0..N {
                if hi[i][j] == '1' {
                    h[i][j] = false;
                }
            }
        }
        for i in 0..N {
            for j in 0..N - 1 {
                if vi[i][j] == '1' {
                    v[i][j] = false;
                }
            }
        }

        Self { N, h, v, d }
    }
}

struct Solver {
    input: Input,
}

impl Solver {
    #[allow(non_upper_case_globals)]
    const dx: [i32; 4] = [0, 1, 0, -1];
    #[allow(non_upper_case_globals)]
    const dy: [i32; 4] = [1, 0, -1, 0];
    const DSTR: [&'static str; 4] = ["R", "D", "L", "U"];

    fn new() -> Self {
        let input = Input::read();
        Self { input }
    }

    fn solve(&self) {
        let mut visited = vec![vec![false; self.input.N]; self.input.N];
        self.dfs(0, 0, &mut visited);

        println!();
    }

    fn dfs(&self, x: i32, y: i32, visited: &mut Vec<Vec<bool>>) {
        visited[x as usize][y as usize] = true;
        for i in 0..4 {
            let nx = x + Self::dx[i];
            let ny = y + Self::dy[i];

            if !self.is_reachable(x, y, i, visited) {
                continue;
            }

            print!("{}", Self::DSTR[i]);
            self.dfs(nx, ny, visited);
            print!("{}", Self::DSTR[(i + 2) % 4]);
        }
    }

    fn is_reachable(&self, x: i32, y: i32, i: usize, visited: &[Vec<bool>]) -> bool {
        let nx = x + Self::dx[i];
        let ny = y + Self::dy[i];

        if nx < 0 || nx >= self.input.N as i32 || ny < 0 || ny >= self.input.N as i32 {
            return false;
        }
        if visited[nx as usize][ny as usize] {
            return false;
        }

        (Self::dx[i] == 0 && self.input.v[x as usize][cmp::min(y, ny) as usize])
            || (Self::dy[i] == 0 && self.input.h[cmp::min(x, nx) as usize][y as usize])
    }
}

fn main() {
    let solver = Solver::new();
    solver.solve();
}
