
//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:447:44

fn main() {
    let _local0 = tui::layout::Rect::new(11565, 11647, 11565, 11565);
    let _local1 = tui::layout::Rect::new(11565, 65407, 0, 11569);
    let _ = tui::layout::Rect::intersection(_local0, _local1);
}
