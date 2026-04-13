use choco_solver::*;
fn main() {
    let model = Model::new(Some("CompilationErrorTest"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);
    let y = model.int_var_bounded(0, 10, Some("y"), None);
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Model>();
}
