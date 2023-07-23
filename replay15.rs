//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:460:16

fn main() {
    let _local0 = tui::layout::Rect::new(13193, 65331, 13107, 13084);
    let _local1 = tui::layout::Rect::new(13130, 65407, 65535, 32);
    let _ = tui::layout::Rect::intersects(_local0, _local1);
}
