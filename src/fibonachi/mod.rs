pub(crate) fn calc(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } 
    return calc(n - 1) + calc(n - 2);
}