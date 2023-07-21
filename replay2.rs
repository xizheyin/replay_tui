//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:434:22
fn main() {
    let _local0 = tui::layout::Rect::new(13624, 12026, 0, 64021);
    let _local1 = tui::layout::Rect::new(20754, 12759, 65535, 32802);
    let _ = tui::layout::Rect::union(_local0, _local1);
}
