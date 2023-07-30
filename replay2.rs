//main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/console-0.15.7/src/common_term.rs:38:50
fn main() {
    let _local5 = console::Term::buffered_stderr();
    let _ = console::Term::move_cursor_to(&(_local5), 18446744073709551615, 18446744069917703941);
}
