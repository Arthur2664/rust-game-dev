fn calculation(n: u32, prev: u32, next: u32, cur: u32) -> u32 {
    if cur == n {
       return next;
    } else {
        return calculation(n, next, prev + next, cur + 1);
    }
}

pub(crate) fn calculate(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    return calculation(n, 1, 1, 2);
}
