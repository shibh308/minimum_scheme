use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::VecDeque;
// use std::time;
use suffix::SuffixTable;

// const TIMEOUT: u64 = 5;

#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
struct CopyFactor {
    sink: usize,
    source: usize,
    len: usize,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize)]
struct CharFactor {
    pos: usize,
    char: char,
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum Factor {
    Copy(CopyFactor),
    Char(CharFactor),
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct State {
    factors: Vec<Factor>,
    group: Vec<i32>,
    used: Vec<bool>,
}

impl Factor {
    fn pos(&self) -> usize {
        match self {
            Factor::Copy(x) => x.sink,
            Factor::Char(x) => x.pos,
        }
    }
}

impl Serialize for Factor {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        match self {
            Factor::Char(x) => serializer.serialize_some(x),
            Factor::Copy(x) => serializer.serialize_some(x),
        }
    }
}

impl PartialOrd for Factor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos().partial_cmp(&other.pos())
    }
}

impl Ord for Factor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos().cmp(&other.pos())
    }
}

impl State {
    fn new(n: usize) -> State {
        State {
            factors: Vec::new(),
            group: vec![-1; n],
            used: vec![false; n],
        }
    }
    fn get_group(&self, x: usize) -> usize {
        if self.group[x] < 0 {
            x
        } else {
            self.get_group(self.group[x] as usize) as usize
        }
    }
    fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.get_group(x);
        let y = self.get_group(y);
        if x == y {
            false
        } else {
            let (x, y) = if self.group[x] > self.group[y] {
                (y, x)
            } else {
                (x, y)
            };
            self.group[x] += self.group[y];
            self.group[y] = x as i32;
            true
        }
    }
}

fn bidirectional_scheme_bfs(
    s: &str,
    s_v: &Vec<char>,
    table: &SuffixTable,
    que: &mut VecDeque<State>,
    state: &State,
) -> bool {
    let n = s.len();
    let target_op = (0..s.len()).find(|x| !state.used[*x]);
    match target_op {
        Some(sink) => {
            let mut new_state = state.clone();
            new_state.factors.push(Factor::Char(CharFactor {
                pos: sink,
                char: s_v[sink],
            }));
            new_state.used[sink] = true;
            que.push_back(new_state);
            if sink + 2 <= n {
                let poses = table
                    .positions(&s[sink..sink + 2])
                    .iter()
                    .map(|x| *x as usize)
                    .collect::<Vec<_>>();
                for source in poses {
                    let mut new_state = state.clone();
                    for len in 1.. {
                        if source.max(sink) + len > n
                            || s_v[source + len - 1] != s_v[sink + len - 1]
                            || new_state.used[sink + len - 1]
                            || !new_state.unite(sink + len - 1, source + len - 1)
                        {
                            break;
                        }
                        new_state.used[sink + len - 1] = true;
                        if len >= 2 {
                            let mut add_state = new_state.clone();
                            add_state
                                .factors
                                .push(Factor::Copy(CopyFactor { sink, source, len }));
                            que.push_back(add_state);
                        }
                    }
                }
            }
            false
        }
        None => true,
    }
}

fn minimum_bidirectional_scheme(s: &str) -> Option<Vec<Vec<Factor>>> {
    let table = suffix::SuffixTable::new(s);
    let s_vec = s.chars().collect();
    let mut que = VecDeque::new();
    que.push_back(State::new(s.len()));
    let mut res = Vec::new();
    // let st = time::Instant::now();
    loop {
        let mut new_que = VecDeque::new();
        while !que.is_empty() {
            /*
            let elapse = st.elapsed();
            if elapse.as_secs() >= TIMEOUT {
                return None;
            }
             */
            let top = que.pop_front().unwrap();
            if bidirectional_scheme_bfs(s, &s_vec, &table, &mut new_que, &top) {
                let mut factors = top.factors;
                factors.sort();
                res.push(factors);
            }
        }
        que = new_que;
        if !res.is_empty() {
            break;
        }
    }
    res.sort();
    Some(res)
}

pub fn calc_min_bs(s: &str) -> Option<String> {
    match minimum_bidirectional_scheme(s) {
        Some(res) => Some(serde_json::to_string(&res).unwrap()),
        None => None,
    }
}
