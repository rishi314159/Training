use std::io;

fn main() {
    let n = getint();
    let k = getint();
    let a = getint();
    let b = getint();

    println!("{}",find_min_cost(n,k,a,b,0));
}

fn getint() -> i64 {
    let mut inpstr = String::new();
    io::stdin().read_line(&mut inpstr).expect("Could not read integer");
    let number = inpstr.trim().parse::<i64>();

    number.unwrap()

}


fn find_min_cost(n : i64, k: i64, a : i64, b : i64, acc: i64) ->  i64 {
    if n == 1 {
        return acc;
    }

    let rem = n%k;

    if rem != 0 {
        return find_min_cost(n - rem, k, a, b, acc+ a*rem);
    }

    let quo = n/k;

    if quo*a + b < n*a {
        return find_min_cost(quo, k, a, b, acc+ b);
    }

    (n-1)*a + acc
}