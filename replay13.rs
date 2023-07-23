//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:459:25

fn main() {
    let _local0 = tui::layout::Rect::new(39578, 39578, 39578, 39578);
    let _local1 = tui::layout::Rect::new(39578, 65535, 65535, 39575);
    let _ = tui::layout::Rect::intersects(_local0, _local1);
}
