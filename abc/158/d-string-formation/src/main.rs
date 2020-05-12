fn main() {
    let (s, q) = input();
    println!("{}", solve(s, q));
}

#[derive(Debug)]
enum Query {
    Reverse,
    Add(Push),
}

// impl Copy for Query { }
// impl Clone for Query {
//     fn clone(&self) -> Query {
//         *self
//     }
// }
#[derive(Debug)]
enum Push {
    Front(String),
    Back(String),
}

fn input() -> (String, Vec<Query>) {
    let mut buf1 = String::new();
    std::io::stdin().read_line(&mut buf1).unwrap();
    buf1.pop();
    let s: String = buf1;
    let mut buf2 = String::new();
    std::io::stdin().read_line(&mut buf2).unwrap();
    buf2.pop();
    let n: usize = buf2.parse().unwrap();

    let mut q: Vec<Query> = Vec::new();
    for _ in 0..n {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.pop();
        let mut iter = buf.split_whitespace();
        let query = iter.next().unwrap();
        match query {
            "1" => q.push(Query::Reverse),
            "2" => {
                let which = iter.next().unwrap();
                match which {
                    "1" => q.push(Query::Add(Push::Front(iter.next().unwrap().to_string()))),
                    "2" => q.push(Query::Add(Push::Back(iter.next().unwrap().to_string()))),
                    _ => {},
                }
            },
            _ => {},
        }
    }
    (s, q)
}

fn solve(s: String, queries: Vec<Query>) -> String {
    let mut rev_count = 0;
    let mut new_queries: Vec<Query> = Vec::new();
    for query in queries.iter() {
        match query {
            Query::Reverse => rev_count += 1,
            Query::Add(Push::Front(c)) => {
                if rev_count % 2 == 0 {
                    new_queries.push(Query::Add(Push::Front(String::from(c))));
                } else {
                    new_queries.push(Query::Reverse);
                    new_queries.push(Query::Add(Push::Front(String::from(c))));
                }
                rev_count = 0;
            },
            Query::Add(Push::Back(c)) => {
                if rev_count % 2 == 0 {
                    new_queries.push(Query::Add(Push::Back(String::from(c))));
                } else {
                    new_queries.push(Query::Reverse);
                    new_queries.push(Query::Add(Push::Back(String::from(c))));
                } 
                rev_count = 0;
            },
            _ => {},
        }
    }
    if rev_count % 2 == 1 {
        new_queries.push(Query::Reverse);
    }
    let mut ans = s;
    let mut rev = false;
    for query in new_queries.iter() {
        match (rev, query) {
            (_, Query::Reverse) => rev = !rev,
            (false, Query::Add(Push::Front(c))) => {
                // push front
                ans = format!("{}{}", c, ans);
            },
            (true, Query::Add(Push::Front(c))) => {
                // push back
                ans.push_str(c);
            },
            (false, Query::Add(Push::Back(c))) => {
                // push back
                ans.push_str(c);
            },
            (true, Query::Add(Push::Back(c))) => {
                // push front
                ans = format!("{}{}", c, ans);
            },
            (_, _) => {},
        }
    }
    if rev {
        return ans.chars().rev().collect::<String>();
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve() {
        let q = vec![
            super::Query::Add(super::Push::Front(String::from("p"))),
            super::Query::Reverse,
            super::Query::Add(super::Push::Back(String::from("c"))),
            super::Query::Reverse,
        ];
        assert_eq!(super::solve(String::from("a"), q), String::from("cpa"));
    }
    #[test]
    fn test_solve2() {
        let q = vec![
            super::Query::Add(super::Push::Back(String::from("a"))),
            super::Query::Add(super::Push::Front(String::from("b"))),
            super::Query::Reverse,
            super::Query::Add(super::Push::Back(String::from("c"))),
            super::Query::Reverse,
            super::Query::Reverse,
        ];
        assert_eq!(super::solve(String::from("a"), q), String::from("aabc"));
    }
}
