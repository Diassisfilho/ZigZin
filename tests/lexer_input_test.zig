pub fn main() void {
    var pi = 3.1415;
    while (true) {
        pi = pi + 2;
        if (pi > 50) {
            break;
        }
    }
    _ = pi + 10 + 2;
}
