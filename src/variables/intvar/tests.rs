use crate::Criterion;
use crate::model::Model;
use crate::variables::intvar::IntVar;
use crate::*;

#[test]
fn test_intvar_creation() {
    let model = Model::new(Some("TestModel"));
    assert_eq!(model.name(), Some("TestModel".to_string()));

    let _int_var = IntVar::new(&model, (1, 10, None));
}

#[test]
fn test_intvar_add() {
    let model = Model::new(Some("AddTestModel"));

    // Create two IntVar instances
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (2, 5, Some("var2")));

    // Add them together
    let result = &var1 + &var2;

    // Verify result bounds are correct
    // result_lb should be var1_lb + var2_lb = 1 + 2 = 3
    // result_ub should be var1_ub + var2_ub = 10 + 5 = 15
    assert_eq!(result.lb(), 3);
    assert_eq!(result.ub(), 15);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(
        solution.get_int_var(&var1).unwrap() + solution.get_int_var(&var2).unwrap()
            == solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_add_constant() {
    let model = Model::new(Some("AddConstantTestModel"));
    // Create an IntVar instance
    let var = IntVar::new(&model, (1, 10, Some("var")));
    // Add a constant to the variable
    let result = &var + 5;
    // Verify result bounds are correct
    // Adding 5 is equivalent to adding +5 int_offset_view
    // result_lb should be var_lb + 5 = 1 + 5 = 6
    // result_ub should be var_ub + 5 = 10 + 5 = 15
    assert_eq!(result.lb(), 6);
    assert_eq!(result.ub(), 15);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(solution.get_int_var(&var).unwrap() + 5 == solution.get_int_var(&result).unwrap());
}

#[test]
fn test_int_var_single_value() {
    let model = Model::new(Some("TestIntVarSingle"));
    let var = IntVar::new(&model, (5, Some("x")));

    assert_eq!(var.lb(), 5);
    assert_eq!(var.ub(), 5);
}

#[test]
fn test_int_var_with_range() {
    let model = Model::new(Some("TestIntVarRange"));
    let var = IntVar::new(&model, (1, 10, Some("y")));

    assert_eq!(var.lb(), 1);
    assert_eq!(var.ub(), 10);
}

#[test]
fn test_int_var_with_array_domain() {
    let model = Model::new(Some("TestIntVarArrayDomain"));
    let var = IntVar::new(&model, (&[1, 3, 5, 7, 9][..], Some("odd")));

    assert_eq!(var.lb(), 1);
    assert_eq!(var.ub(), 9);
}

#[test]
fn test_int_var_with_bounded_domain_true() {
    let model = Model::new(Some("TestIntVarBoundedTrue"));
    let var = IntVar::new(&model, (1, 5, Some("bounded"), true));

    assert_eq!(var.lb(), 1);
    assert_eq!(var.ub(), 5);
    assert!(!var.has_enumerated_domain());
    assert_eq!(var.get_domain_values(), None);
}

#[test]
fn test_int_var_with_bounded_domain_false() {
    let model = Model::new(Some("TestIntVarBoundedFalse"));
    let var = IntVar::new(&model, (1, 5, Some("enumerated"), false));

    assert_eq!(var.lb(), 1);
    assert_eq!(var.ub(), 5);
    assert!(var.has_enumerated_domain());
    assert_eq!(var.get_domain_values(), Some(vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_int_var_vec_domain() {
    let model = Model::new(Some("TestIntVarVec"));
    let domain = [2, 4, 6, 8, 10];
    let var = IntVar::new(&model, (&domain[..], Some("even")));

    assert_eq!(var.lb(), 2);
    assert_eq!(var.ub(), 10);
}

#[test]
fn test_intvar_sub_intvar() {
    let model = Model::new(Some("SubTestModel"));

    // Create two IntVar instances
    let var1 = IntVar::new(&model, (10, 20, Some("var1")));
    let var2 = IntVar::new(&model, (2, 5, Some("var2")));

    // Subtract var2 from var1
    let result = &var1 - &var2;

    // Verify result bounds are correct
    // result_lb should be var1_lb - var2_ub = 10 - 5 = 5
    // result_ub should be var1_ub - var2_lb = 20 - 2 = 18
    assert_eq!(result.lb(), 5);
    assert_eq!(result.ub(), 18);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(
        solution.get_int_var(&var1).unwrap() - solution.get_int_var(&var2).unwrap()
            == solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_sub_constant() {
    let model = Model::new(Some("SubConstantTestModel"));

    // Create an IntVar instance
    let var = IntVar::new(&model, (10, 20, Some("var")));

    // Subtract a constant from the variable
    let result = &var - 5;

    // Verify result bounds are correct
    // Subtracting 5 is equivalent to adding -5 offset
    // result_lb should be var_lb - 5 = 10 - 5 = 5
    // result_ub should be var_ub - 5 = 20 - 5 = 15
    assert_eq!(result.lb(), 5);
    assert_eq!(result.ub(), 15);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(solution.get_int_var(&var).unwrap() - 5 == solution.get_int_var(&result).unwrap());
}

#[test]
fn test_intvar_mul_intvar() {
    let model = Model::new(Some("MulTestModel"));

    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (2, 5, Some("var2")));

    let result = &var1 * &var2;

    assert_eq!(result.lb(), 2);
    assert_eq!(result.ub(), 50);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(
        solution.get_int_var(&var1).unwrap() * solution.get_int_var(&var2).unwrap()
            == solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_mul_constant() {
    let model = Model::new(Some("MulConstantTestModel"));

    let var = IntVar::new(&model, (1, 10, Some("var")));
    let result = &var * 3;

    assert_eq!(result.lb(), 3);
    assert_eq!(result.ub(), 30);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(solution.get_int_var(&var).unwrap() * 3 == solution.get_int_var(&result).unwrap());
}

#[test]
fn test_intvar_div_intvar() {
    let model = Model::new(Some("DivTestModel"));

    let var1 = IntVar::new(&model, (10, 20, Some("var1")));
    let var2 = IntVar::new(&model, (2, 5, Some("var2")));

    let result = &var1 / &var2;

    assert_eq!(result.lb(), 2);
    assert_eq!(result.ub(), 10);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(
        solution.get_int_var(&var1).unwrap() / solution.get_int_var(&var2).unwrap()
            == solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_div_constant() {
    let model = Model::new(Some("DivConstantTestModel"));

    let var = IntVar::new(&model, (10, 20, Some("var")));
    let result = &var / 2;

    assert_eq!(result.lb(), 5);
    assert_eq!(result.ub(), 10);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(solution.get_int_var(&var).unwrap() / 2 == solution.get_int_var(&result).unwrap());
}

#[test]
fn test_intvar_neg() {
    let model = Model::new(Some("NegTestModel"));

    let var = IntVar::new(&model, (1, 10, Some("var")));
    let result = -&var;

    assert_eq!(result.lb(), -10);
    assert_eq!(result.ub(), -1);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert_eq!(
        -solution.get_int_var(&var).unwrap(),
        solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_rem_constant() {
    let model = Model::new(Some("RemConstantTestModel"));

    let var = IntVar::new(&model, (4, 5, Some("var")));
    let result = &var % 3;

    assert_eq!(result.lb(), 1);
    assert_eq!(result.ub(), 2);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert_eq!(
        solution.get_int_var(&var).unwrap() % 3,
        solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_intvar_rem_intvar() {
    let model = Model::new(Some("RemTestModel"));

    let var1 = IntVar::new(&model, (10, 20, Some("var1")));
    let var2 = IntVar::new(&model, (3, 7, Some("var2")));

    let result = &var1 % &var2;

    assert_eq!(result.lb(), 1);
    assert_eq!(result.ub(), 6);
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert_eq!(
        solution.get_int_var(&var1).unwrap() % solution.get_int_var(&var2).unwrap(),
        solution.get_int_var(&result).unwrap()
    );
}

#[test]
fn test_reify_eq_y() {
    let model = Model::new(Some("ReifyXEqYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_eq_y(&y, &b);

    // Post additional constraints to test reification
    b.eq(true).post().unwrap();
    x.eq(3).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_eq_y(3, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert_eq!(
        solution.get_int_var(&x).unwrap(),
        solution.get_int_var(&y).unwrap()
    );
    assert!(solution.get_bool_var(&b).unwrap());
    assert_eq!(solution.get_int_var(&x2).unwrap(), 3);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_ne_y() {
    let model = Model::new(Some("ReifyXNeYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_ne_y(&y, &b);
    b.eq(true).post().unwrap();
    x.eq(3).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_ne_y(3, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert_ne!(
        solution.get_int_var(&x).unwrap(),
        solution.get_int_var(&y).unwrap()
    );
    assert!(solution.get_bool_var(&b).unwrap());
    assert_ne!(solution.get_int_var(&x2).unwrap(), 3);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_lt_y() {
    let model = Model::new(Some("ReifyXLtYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_lt_y(&y, &b);
    b.eq(true).post().unwrap();
    x.eq(2).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_lt_y(4, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() < solution.get_int_var(&y).unwrap());
    assert!(solution.get_bool_var(&b).unwrap());
    assert!(solution.get_int_var(&x2).unwrap() < 4);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_gt_y() {
    let model = Model::new(Some("ReifyXGtYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_gt_y(&y, &b);
    b.eq(true).post().unwrap();
    x.eq(4).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_gt_y(2, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() > solution.get_int_var(&y).unwrap());
    assert!(solution.get_bool_var(&b).unwrap());
    assert!(solution.get_int_var(&x2).unwrap() > 2);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_le_y() {
    let model = Model::new(Some("ReifyXLeYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_le_y(&y, &b);
    b.eq(true).post().unwrap();
    x.eq(3).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_le_y(4, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() <= solution.get_int_var(&y).unwrap());
    assert!(solution.get_bool_var(&b).unwrap());
    assert!(solution.get_int_var(&x2).unwrap() <= 4);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_ge_y() {
    let model = Model::new(Some("ReifyXGeYTestModel"));

    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_ge_y(&y, &b);
    b.eq(true).post().unwrap();
    x.eq(3).post().unwrap();

    // Test with i32
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let b2 = model.bool_var(None, Some("b2"));

    x2.reify_ge_y(2, &b2);
    b2.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() >= solution.get_int_var(&y).unwrap());
    assert!(solution.get_bool_var(&b).unwrap());
    assert!(solution.get_int_var(&x2).unwrap() >= 2);
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_reify_eq_yc() {
    let model = Model::new(Some("ReifyXEqYcTestModel"));

    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_eq_yc(&y, 3, &b);
    b.eq(true).post().unwrap();
    y.eq(2).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert_eq!(
        solution.get_int_var(&x).unwrap(),
        solution.get_int_var(&y).unwrap() + 3
    );
    assert!(solution.get_bool_var(&b).unwrap());
}

#[test]
fn test_reify_ne_yc() {
    let model = Model::new(Some("ReifyXNeYcTestModel"));

    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_ne_yc(&y, 3, &b);
    b.eq(true).post().unwrap();
    x.eq(2).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert_ne!(
        solution.get_int_var(&x).unwrap(),
        solution.get_int_var(&y).unwrap() + 3
    );
    assert!(solution.get_bool_var(&b).unwrap());
}

#[test]
fn test_reify_lt_yc() {
    let model = Model::new(Some("ReifyXLtYcTestModel"));

    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_lt_yc(&y, 3, &b);
    b.eq(true).post().unwrap();
    x.eq(2).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() < solution.get_int_var(&y).unwrap() + 3);
    assert!(solution.get_bool_var(&b).unwrap());
}

#[test]
fn test_reify_gt_yc() {
    let model = Model::new(Some("ReifyXGtYcTestModel"));

    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(1, 5, Some("y"), None);
    let b = model.bool_var(None, Some("b"));

    x.reify_gt_yc(&y, 3, &b);
    b.eq(true).post().unwrap();
    x.eq(8).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    assert!(solution.get_int_var(&x).unwrap() > solution.get_int_var(&y).unwrap() + 3);
    assert!(solution.get_bool_var(&b).unwrap());
}

#[test]
fn test_member_table() {
    let model = Model::new(Some("MemberTableTestModel"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);

    x.member_table(&[2, 4, 6, 8]).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let val = solution.get_int_var(&x).unwrap();
    assert!([2, 4, 6, 8].contains(&val));
}

#[test]
fn test_member_bounds() {
    let model = Model::new(Some("MemberBoundsTestModel"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);

    x.member_bounds(3, 7).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let val = solution.get_int_var(&x).unwrap();
    assert!((3..=7).contains(&val));
}

#[test]
fn test_not_member_table() {
    let model = Model::new(Some("NotMemberTableTestModel"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);

    x.not_member_table(&[1, 3, 5]).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let val = solution.get_int_var(&x).unwrap();
    assert!(![1, 3, 5].contains(&val));
}

#[test]
fn test_not_member_bounds() {
    let model = Model::new(Some("NotMemberBoundsTestModel"));
    let x = model.int_var_bounded(0, 10, Some("x"), None);

    x.not_member_bounds(4, 6).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let val = solution.get_int_var(&x).unwrap();
    assert!(!(4..=6).contains(&val));
}

#[test]
fn test_abs() {
    let model = Model::new(Some("AbsTestModel"));
    let x = model.int_var_bounded(-10, 10, Some("x"), None);
    let y = model.int_var_bounded(0, 10, Some("y"), None);

    y.abs(&x).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    assert_eq!(y_val, x_val.abs());
}

#[test]
fn test_square() {
    let model = Model::new(Some("SquareTestModel"));
    let x = model.int_var_bounded(1, 5, Some("x"), None);
    let y = model.int_var_bounded(0, 30, Some("y"), None);

    y.square(&x).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    assert_eq!(y_val, x_val * x_val);
}

#[test]
fn test_pow() {
    let model = Model::new(Some("PowTestModel"));
    let x = model.int_var_bounded(2, 4, Some("x"), None);
    let y = model.int_var_bounded(1, 100, Some("y"), None);

    x.pow(3, &y).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    assert_eq!(y_val, x_val.pow(3));
}

#[test]
fn test_max() {
    let model = Model::new(Some("MaxTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(5, 15, Some("y"), None);
    let z = model.int_var_bounded(1, 15, Some("z"), None);

    z.max(&[&x, &y]).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    let z_val = solution.get_int_var(&z).unwrap();
    assert_eq!(z_val, x_val.max(y_val));
}

#[test]
fn test_min() {
    let model = Model::new(Some("MinTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);
    let y = model.int_var_bounded(5, 15, Some("y"), None);
    let z = model.int_var_bounded(1, 15, Some("z"), None);

    z.min(&[&x, &y]).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();
    let z_val = solution.get_int_var(&z).unwrap();
    assert_eq!(z_val, x_val.min(y_val));
}
#[test]
fn test_among() {
    let model = Model::new(Some("AmongTestModel"));
    let count = model.int_var_bounded(0, 5, Some("count"), None);
    let x1 = model.int_var_bounded(1, 10, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 10, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 10, Some("x3"), None);
    let x4 = model.int_var_bounded(1, 10, Some("x4"), None);

    // count should be the number of variables in [x1, x2, x3, x4] that take values in [2, 4, 6]
    count
        .among(&[&x1, &x2, &x3, &x4], &[2, 4, 6])
        .post()
        .unwrap();

    // Force specific values for testing
    x1.eq(2).post().unwrap(); // in values
    x2.eq(4).post().unwrap(); // in values
    x3.eq(5).post().unwrap(); // not in values
    x4.eq(6).post().unwrap(); // in values

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let count_val = solution.get_int_var(&count).unwrap();
    let x1_val = solution.get_int_var(&x1).unwrap();
    let x2_val = solution.get_int_var(&x2).unwrap();
    let x3_val = solution.get_int_var(&x3).unwrap();
    let x4_val = solution.get_int_var(&x4).unwrap();

    // Count how many variables take values in [2, 4, 6]
    let expected_count = [x1_val, x2_val, x3_val, x4_val]
        .iter()
        .filter(|&&v| [2, 4, 6].contains(&v))
        .count();

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let expected_count_i32 = expected_count as i32;
    assert_eq!(count_val, expected_count_i32);
    assert_eq!(count_val, 3); // x1=2, x2=4, x4=6 are in values
}
#[test]
fn test_all_different() {
    let model = Model::new(Some("AllDifferentTestModel"));
    let x1 = model.int_var_bounded(1, 5, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 5, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 5, Some("x3"), None);

    // Post all_different constraint on the slice
    (&[&x1, &x2, &x3][..]).all_different().post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x1_val = solution.get_int_var(&x1).unwrap();
    let x2_val = solution.get_int_var(&x2).unwrap();
    let x3_val = solution.get_int_var(&x3).unwrap();

    // All values must be different
    assert_ne!(x1_val, x2_val);
    assert_ne!(x2_val, x3_val);
    assert_ne!(x1_val, x3_val);
}

#[test]
fn test_all_different_except_0() {
    let model = Model::new(Some("AllDifferentExcept0TestModel"));
    let x1 = model.int_var_bounded(0, 5, Some("x1"), None);
    let x2 = model.int_var_bounded(0, 5, Some("x2"), None);
    let x3 = model.int_var_bounded(0, 5, Some("x3"), None);
    let x4 = model.int_var_bounded(0, 5, Some("x4"), None);

    // Post all_different_except_0 constraint: allows multiple 0 values
    (&[&x1, &x2, &x3, &x4][..])
        .all_different_except_0()
        .post()
        .unwrap();

    // Set some variables to 0
    x1.eq(0).post().unwrap();
    x2.eq(0).post().unwrap();
    x3.eq(1).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x1_val = solution.get_int_var(&x1).unwrap();
    let x2_val = solution.get_int_var(&x2).unwrap();
    let x3_val = solution.get_int_var(&x3).unwrap();
    let x4_val = solution.get_int_var(&x4).unwrap();

    // Both x1 and x2 can be 0
    assert_eq!(x1_val, 0);
    assert_eq!(x2_val, 0);
    assert_eq!(x3_val, 1);

    // x4 must be different from x3 (since x3 is not 0)
    // and can be 0 or different from other non-zero values
    assert!(x4_val == 0 || (x4_val != x1_val && x4_val != x3_val));
}
#[test]
fn test_eq_view() {
    let model = Model::new(Some("EqViewTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);

    // Create a boolean view representing x == 5
    let b_eq_5 = x.eq_view(5);

    x.eq(5).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let b_val = solution.get_bool_var(&b_eq_5).unwrap();

    assert_eq!(x_val, 5);
    assert!(b_val); // b_eq_5 should be true when x == 5
}

#[test]
fn test_ne_view() {
    let model = Model::new(Some("NeViewTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);

    // Create a boolean view representing x != 3
    let b_ne_3 = x.ne_view(3);

    x.ne(3).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let b_val = solution.get_bool_var(&b_ne_3).unwrap();

    assert_ne!(x_val, 3);
    assert!(b_val); // b_ne_3 should be true when x != 3
}

#[test]
fn test_le_view() {
    let model = Model::new(Some("LeViewTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);

    // Create a boolean view representing x <= 7
    let b_le_7 = x.le_view(7);

    // Use arithmetic constraint for <= comparison
    x.arithm(EqualityOperator::Leq, 7).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let b_val = solution.get_bool_var(&b_le_7).unwrap();

    assert!(x_val <= 7);
    assert!(b_val); // b_le_7 should be true when x <= 7
}

#[test]
fn test_ge_view() {
    let model = Model::new(Some("GeViewTestModel"));
    let x = model.int_var_bounded(1, 10, Some("x"), None);

    // Create a boolean view representing x >= 4
    let b_ge_4 = x.ge_view(4);

    // Use arithmetic constraint for >= comparison
    x.arithm(EqualityOperator::Geq, 4).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x_val = solution.get_int_var(&x).unwrap();
    let b_val = solution.get_bool_var(&b_ge_4).unwrap();

    assert!(x_val >= 4);
    assert!(b_val); // b_ge_4 should be true when x >= 4
}
#[test]
fn test_all_equal() {
    let model = Model::new(Some("AllEqualTestModel"));
    let x1 = model.int_var_bounded(1, 10, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 10, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 10, Some("x3"), None);

    // Post all_equal constraint on the slice
    (&[&x1, &x2, &x3][..]).all_equal().post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x1_val = solution.get_int_var(&x1);
    let x2_val = solution.get_int_var(&x2);
    let x3_val = solution.get_int_var(&x3);

    // All values must be equal
    assert_eq!(x1_val, x2_val);
    assert_eq!(x2_val, x3_val);
}

#[test]
fn test_not_all_equal() {
    let model = Model::new(Some("NotAllEqualTestModel"));
    let x1 = model.int_var_bounded(1, 10, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 10, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 10, Some("x3"), None);

    // Post not_all_equal constraint: ensures not all variables take the same value
    (&[&x1, &x2, &x3][..]).not_all_equal().post().unwrap();

    // Force x1 and x2 to be equal
    x1.eq(5).post().unwrap();
    x2.eq(5).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x1_val = solution.get_int_var(&x1);
    let x2_val = solution.get_int_var(&x2);
    let x3_val = solution.get_int_var(&x3);

    // x1 and x2 are equal, but x3 must be different
    assert_eq!(x1_val, x2_val);
    assert_ne!(x3_val, x1_val);
}

#[test]
fn test_at_least_n_value() {
    let model = Model::new(Some("AtLeastNValueTestModel"));
    let count = model.int_var_bounded(0, 5, Some("count"), None);
    let x1 = model.int_var_bounded(1, 10, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 10, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 10, Some("x3"), None);

    // count should be at least 2 (at least 2 distinct values among x1, x2, x3)
    (&[&x1, &x2, &x3][..])
        .at_least_n_value(&count, false)
        .post()
        .unwrap();

    // Force specific values to have exactly 2 distinct values
    x1.eq(5).post().unwrap();
    x3.eq(5).post().unwrap();
    count.eq(2).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x2_val = solution.get_int_var(&x2).unwrap();
    assert_ne!(x2_val, 5);
}

#[test]
fn test_at_most_n_value() {
    let model = Model::new(Some("AtMostNValueTestModel"));
    let count = model.int_var_bounded(0, 5, Some("count"), None);
    let x1 = model.int_var_bounded(1, 10, Some("x1"), None);
    let x2 = model.int_var_bounded(1, 10, Some("x2"), None);
    let x3 = model.int_var_bounded(1, 10, Some("x3"), None);

    // count should be at most 2 (at most 2 distinct values among x1, x2, x3)
    (&[&x1, &x2, &x3][..])
        .at_most_n_value(&count, false)
        .post()
        .unwrap();

    // Force specific values to have exactly 2 distinct values
    x1.eq(3).post().unwrap();
    x3.eq(9).post().unwrap();
    count.eq(2).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    let x2_val = solution.get_int_var(&x2).unwrap();
    assert!(x2_val == 3 || x2_val == 9); // x2 must be either 3 or 9 to ensure at most 2 distinct values
}
