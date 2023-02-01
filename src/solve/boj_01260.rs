use std::collections::VecDeque;
use std::io;
use std::io::Write;

pub fn main(){
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Fail");
    let info: Vec<i32> = buf.split_whitespace().map(|x| x.parse().expect("Fail")).collect();
    buf.clear();
    let mut data : Vec<Vec<u16>> = vec![vec![];(info[0]+1) as usize];

    for _i in 0..info[1]{
        io::stdin().read_line(&mut buf).expect("Fail");
        let tmp: Vec<u16> = buf.split_whitespace().map(|x| x.parse().expect("Fail")).collect();
        buf.clear();
        data[tmp[0] as usize].push(tmp[1]);
        data[tmp[1] as usize].push(tmp[0]);
    }
    for i in 1..=info[0]{
        data[i as usize].sort_by(|a, b| b.partial_cmp(a).unwrap());
    }
    write!(out,"{}{}",dfs(&info, &mut data).trim_end(),"\n").unwrap();
    write!(out,"{}",bfs(&info, &mut data).trim_end()).unwrap();
}

fn dfs(info : &Vec<i32>, data : &mut Vec<Vec<u16>>) -> String{
    let mut st : Vec<u16> = vec![info[2] as u16];
    let mut visited = vec![false;1001];
    let mut ret = String::new();
    loop {
        if st.is_empty() {
            break;
        }
        let cur = st.pop().unwrap();
        if visited[cur as usize] {
            continue;
        }
        visited[cur as usize] = true;
        ret.push_str(&cur.to_string());
        ret.push_str(" ");
        for i in &data[cur as usize] {
            if !visited[*i as usize] {
                st.push(*i);
            }
        }
    }
    ret
}

fn bfs(info : &Vec<i32>, data : &mut Vec<Vec<u16>>) -> String {
    let mut st : VecDeque<u16> = VecDeque::new();
    st.push_front(info[2] as u16);
    let mut ret = String::new();
    for i in 1..=info[0]{
        data[i as usize].sort();
    }
    let mut visited = vec![false;1001];
    loop {
        if st.is_empty() {
            break;
        }
        let cur = st.pop_back().unwrap();
        if visited[cur as usize] {
            continue;
        }
        visited[cur as usize] = true;
        ret.push_str(&cur.to_string());
        ret.push_str(" ");
        for i in &data[cur as usize] {
            if !visited[*i as usize] {
                st.push_front(*i);
            }
        }
    }
    ret
}