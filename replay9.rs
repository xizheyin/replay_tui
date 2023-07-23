//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:446:22
fn main() {
    let _local0 = tui::layout::Rect::new(11566, 11558, 59394, 0);
    let _local1 = tui::layout::Rect::new(11520, 32557, 4096, 64);
    let _ = tui::layout::Rect::intersection(_local0, _local1);
}
