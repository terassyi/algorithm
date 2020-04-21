fn main() {
    let (n, k) = input();
    println!("{}", solve(n, k));
}

fn input() -> (usize, usize) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.pop();
    let mut buf = buf.split_whitespace();
    let n: usize = buf.next().unwrap().parse().unwrap();
    let k: usize = buf.next().unwrap().parse().unwrap();
    (n, k)
}

fn solve(n: usize, k: usize) -> usize {
    let mut ans: usize = 0;
    for m in k..n+2 {
        let min = m*(m-1)/2;
        let max = (2*n-m+1)*m/2;
        ans += max - min + 1;
    }
    ans % 1000000007
}

/*
10^100は非常に巨大なのでM個の数の和はほぼM*100^100となる．
なので，選んだ数の個数がK,K+1,...,N+1の各場合に答えを求めて，その和をとれば良い．
M個の数を選ぶ時，和としてあり得る最小値は小さい方からM個とった場合．最大値は大きい方からM個とった場合．
最小値と最大値の間の値は全て作ることができる．
よって，最大値-最小値を求めることができれば良い．
*/
