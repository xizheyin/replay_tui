//thread 'main' panicked at 'attempt to add with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:434:44

fn main() {
    let _local0 = tui::layout::Rect::new(13569, 0, 0, 12594);
    let _local1 = tui::layout::Rect::new(14642, 12544, 0, 65535);
    let _ = tui::layout::Rect::union(_local0, _local1);
}
