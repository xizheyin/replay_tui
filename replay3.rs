//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:433:22
fn main() {
    let _local0 = tui::layout::Rect::new(64050, 12593, 11828, 50);
    let _local1 = tui::layout::Rect::new(14592, 5626, 13614, 13312);
    let _ = tui::layout::Rect::union(_local0, _local1);
}
