
//thread 'main' panicked at 'attempt to subtract with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:452:21

fn main() {
    let _local0 = tui::layout::Rect::new(15934, 13879, 12846, 17982);
    let _local1 = tui::layout::Rect::new(15934, 19248, 13652, 13876);
    let _ = tui::layout::Rect::intersection(_local0, _local1);
}
