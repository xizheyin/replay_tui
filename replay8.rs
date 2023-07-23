//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:447:22

fn main() {
    let _local0 = tui::layout::Rect::new(25471, 65325, 25471, 65423);
    let _local1 = tui::layout::Rect::new(36751, 10301, 11821, 11569);
    let _ = tui::layout::Rect::intersection(_local0, _local1);
}
