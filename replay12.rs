//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:457:18

fn main() {
    let _local0 = tui::layout::Rect::new(30262, 6964, 32767, 65535);
    let _local1 = tui::layout::Rect::new(62719, 100, 32767, 48);
    let _ = tui::layout::Rect::intersects(_local0, _local1);
}
