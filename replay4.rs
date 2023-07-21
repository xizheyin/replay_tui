//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:433:43

fn main() {
    let _local0 = tui::layout::Rect::new(14641, 65535, 65535, 65535);
    let _local1 = tui::layout::Rect::new(65535, 61183, 65535, 65333);
    let _ = tui::layout::Rect::union(_local0, _local1);
}
