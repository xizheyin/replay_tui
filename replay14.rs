//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:458:16

fn main() {
    let _local0 = tui::layout::Rect::new(12852, 4895, 65280, 0);
    let _local1 = tui::layout::Rect::new(32767, 4895, 0, 1);
    let _ = tui::layout::Rect::intersects(_local0, _local1);
}
