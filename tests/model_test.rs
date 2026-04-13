use choco_solver::*;
#[test]
fn test_model_creation() {
    let model = Model::new(Some("TestModel"));
    assert_eq!(model.name(), Some("TestModel".to_string()));
    // var1 <=> var2 + var3 == 156
    let var2 = model.int_var_bounded(0, 200, Some("var2"), None);
    let var3 = model.int_var_bounded(0, 200, Some("var3"), None);
    let var1 = (&var2 + &var3).eq(156).reify();
    let solver = model.solver();
    {
        let solution = solver
            .find_solution(&Criterion::default())
            .expect("failed to find solution");
        let val_var1 = solution.get_bool_var(&var1).unwrap();
        let val_var2 = solution.get_int_var(&var2).unwrap();
        let val_var3 = solution.get_int_var(&var3).unwrap();
        assert_eq!(val_var1, (val_var2 + val_var3) == 156);
        println!(
            "Solution found: var1 = {}, var2 = {}, var3 = {}",
            val_var1, val_var2, val_var3
        );
        assert!(var2.is_instantiated()); //
        assert!(var3.is_instantiated());
    }
    true.eq(&var1).post().unwrap();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    let val_var1 = solution.get_bool_var(&var1).unwrap();
    let val_var2 = solution.get_int_var(&var2).unwrap();
    let val_var3 = solution.get_int_var(&var3).unwrap();
    assert_eq!(val_var1, (val_var2 + val_var3) == 156);
    println!(
        "Solution found: var1 = {}, var2 = {}, var3 = {}",
        val_var1, val_var2, val_var3
    );

    //
}

#[test]
pub fn test_reify_functions() {
    let model = Model::new(Some("ReifyTest"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);
    let y = model.int_var_bounded(0, 10, Some("y"), None);
    let b = model.bool_var(None, None);
    x.reify_eq_y(1, &b);
    x.reify_eq_y(&y, &b);
    b.eq(true).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("Expected to find a solution");
    let b_val = solution.get_bool_var(&b).unwrap();
    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    assert!(b_val);
    assert_eq!(x_val, 1);
    assert_eq!(y_val, 1);
}

#[test]
fn test_compilation_error() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fails/*.rs");
}

// #[test]
// fn test_mixed_model() {
//     let model1 = Model::new(Some("MixedModel1"));
//     let model2 = Model::new(Some("MixedModel2"));
//     let int_var1 = model1.int_var_bounded(0, 100, Some("int_var1"), None);
//     let int_var2 = model2.int_var_bounded(50, 150, Some("int_var2"), None);
//     (&int_var1 + &int_var2).eq(55).post().unwrap();

//     let solver1 = model1.solver();
//     let solver2 = model2.solver();
//     {
//         let solution1 = solver1
//             .find_solution(&Criterion::default())
//             .expect("failed to find solution in model1");
//         let val_int_var1 = solution1.get_int_var(&int_var1);
//         let val_int_var2 = solution1.get_int_var(&int_var2);
//         println!(
//             "Model1 Solution: int_var1 = {} int_var2 = {}",
//             val_int_var1, val_int_var2
//         );
//         assert!(int_var1.is_instantiated());
//     }
//     {
//         let solution2 = solver2
//             .find_solution(&Criterion::default())
//             .expect("failed to find solution in model2");
//         let val_int_var2 = solution2.get_int_var(&int_var2);
//         let val_int_var1 = solution2.get_int_var(&int_var1);
//         println!(
//             "Model2 Solution: int_var2 = {} int_var1 = {}",
//             val_int_var2, val_int_var1
//         );
//         assert!(int_var2.is_instantiated());
//     }
// }
