//thread 'main' panicked at 'attempt to subtract with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:451:20

fn main() {
    let _local0 = tui::layout::Rect::new(60543, 0, 0, 0);
    let _local1 = tui::layout::Rect::new(0, 0, 0, 0);
    let _ = tui::layout::Rect::intersection(_local0, _local1);
}
