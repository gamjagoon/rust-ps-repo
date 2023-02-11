use std::io;
use std::io::Write;
use std::collections::VecDeque;

pub fn main(){
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Fail");
    let n: Vec<usize> = buf.split_whitespace().map(|x| x.parse().expect("Fail")).collect();
    buf.clear();
    let mut mmap : Vec<Vec<usize>> = vec![vec![];n[0]];
    for r in 0..n[0] {
        io::stdin().read_line(&mut buf).expect("Fail");
        let tmp = buf.trim_end_matches('\n');
        for c in tmp.chars() {
            mmap[r].push(c.to_digit(10).unwrap() as usize);
        }
        buf.clear();
    }
    write!(out, "{}", bfs(&mut mmap,n[0],n[1])).unwrap();
}

fn bfs(mmap : &mut Vec<Vec<usize>>, r : usize, c : usize) -> usize {
    let ret : usize = 0;
    let mut q : VecDeque<(usize, usize, usize)> = VecDeque::new();
    q.push_front((0, 0, 1));

    let mut cur_r:usize;
    let mut cur_c:usize; 
    let mut path_len:usize; 
    while !q.is_empty() {
        (cur_r, cur_c, path_len) = q.pop_back().unwrap();
        if cur_r == r-1 && cur_c == c-1 {
            return path_len ;
        }
        if cur_r > 0 {
            if mmap[cur_r-1][cur_c] == 1 {
                mmap[cur_r-1][cur_c] = 0;
                q.push_front((cur_r-1, cur_c, path_len+1));
            }
        }
        if cur_r < r - 1 {
            if mmap[cur_r+1][cur_c] == 1 {
                mmap[cur_r+1][cur_c] = 0;
                q.push_front((cur_r + 1, cur_c, path_len+1));
            }
        }
        if cur_c > 0 {
            if mmap[cur_r][cur_c-1] == 1 {
                mmap[cur_r][cur_c-1] = 0;
                q.push_front((cur_r, cur_c-1, path_len+1));
            }
        }
        
        if cur_c < c - 1 {
            if mmap[cur_r][cur_c+1] == 1 {
                mmap[cur_r][cur_c+1] = 0;
                q.push_front((cur_r, cur_c+1, path_len+1));
            }
        }
    }

    ret
}