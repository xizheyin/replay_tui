//thread 'main' panicked at 'attempt to multiply with overflow', /home/yxz/.cargo/registry/src/github.com-1ecc6299db9ec823/tui-0.19.0/src/layout.rs:398:9
fn main() {
    let _local0 = tui::layout::Rect::new(0, 25826, 11763, 0);
    let mut _local1 = tui::buffer::Buffer::empty(_local0);
    let _local2 = tui::backend::TestBackend::new(5759, 0);
    let _local3 = tui::backend::TestBackend::buffer(&(_local2));
    let _ = tui::buffer::Buffer::merge(&mut (_local1), _local3);
}
