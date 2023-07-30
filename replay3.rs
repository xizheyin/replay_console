//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/console-0.15.7/src/common_term.rs:38:43
fn main() {
    let _local2 = console::Term::buffered_stdout();
    let _ = console::Term::move_cursor_to(&(_local2), 18446744073172674815, 18446744073709551615);
}
