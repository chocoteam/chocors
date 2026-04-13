use crate::ConstraintEquality;
use crate::Criterion;
use crate::model::Model;
use crate::variables::intvar::{BoolVar, IntVar};
use crate::*;

#[test]
fn test_bool_eq_boolvar() {
    let model = Model::new(Some("TestBoolEqBoolVar"));
    let bool_var = BoolVar::new(&model, None, Some("b"));

    // Test: true == bool_var
    let result = true.eq(&bool_var).reify();

    // Post constraint and solve
    true.eq(&bool_var).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // When bool_var = true, result should be true
    assert!(solution.get_bool_var(&bool_var).unwrap());
    assert!(solution.get_bool_var(&result).unwrap());
    assert_eq!(bool_var.name(), "b");
}

#[test]
fn test_bool_ne_boolvar() {
    let model = Model::new(Some("TestBoolNeBoolVar"));
    let bool_var = BoolVar::new(&model, None, None);

    // Test: false != bool_var
    let result = false.ne(&bool_var).reify();

    // Post constraint and solve
    true.eq(&bool_var).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // When bool_var = true, result should be true because false != true
    assert!(solution.get_bool_var(&bool_var).unwrap());
    println!("in instantiated {}", result.is_instantiated());
    println!(
        "Result of false != bool_var: {}",
        solution.get_bool_var(&result).unwrap(),
    );
    assert!(solution.get_bool_var(&result).unwrap());
}

#[test]
fn test_boolvar_eq_boolvar() {
    let model = Model::new(Some("TestBoolVarEqBoolVar"));
    let bool_var1 = BoolVar::new(&model, None, Some("b1"));
    let bool_var2 = BoolVar::new(&model, None, Some("b2"));

    // Test: bool_var1 == bool_var2
    let result = bool_var1.eq(&bool_var2).reify();

    // Post constraint and solve
    true.eq(&bool_var1).post().unwrap();
    true.eq(&bool_var2).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // When both are true, result should be true
    assert!(solution.get_bool_var(&bool_var1).unwrap());
    assert!(solution.get_bool_var(&bool_var2).unwrap());
    assert!(solution.get_bool_var(&result).unwrap());
}

#[test]
fn test_boolvar_ne_boolvar() {
    let model = Model::new(Some("TestBoolVarNeBoolVar"));
    let bool_var1 = BoolVar::new(&model, None, Some("b1"));
    let bool_var2 = BoolVar::new(&model, None, Some("b2"));

    // Test: bool_var1 != bool_var2
    let result = (&bool_var1).ne(&bool_var2).reify();

    // Post constraint and solve
    true.eq(&bool_var1).post().unwrap();
    false.eq(&bool_var2).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // When b1=true and b2=false, result should be true because they're different
    assert!(solution.get_bool_var(&bool_var1).unwrap());
    assert!(!solution.get_bool_var(&bool_var2).unwrap());
    assert!(solution.get_bool_var(&result).unwrap());
}

#[test]
fn test_bool_not_view() {
    let model = Model::new(Some("TestBoolNotView"));
    let bool_var = BoolVar::new(&model, None, Some("b"));

    // Create NOT view
    let not_view = !&bool_var;

    // Post constraint and solve
    false.eq(&bool_var).post().unwrap();
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // When bool_var = false, not_view should be true
    assert!(!solution.get_bool_var(&bool_var).unwrap());
    assert!(solution.get_bool_var(&not_view).unwrap());
}

#[test]
fn test_logical_trait_array() {
    let model = Model::new(Some("TestLogicalTraitArray"));
    let bool1 = model.bool_var(None, None);
    let bool2 = model.bool_var(None, None);
    [&bool1, &bool2]
        .and()
        .post()
        .expect("failed to post logical AND constraint");

    let bool3 = model.bool_var(None, None);
    let bool4 = model.bool_var(None, None);
    [&bool3, &bool4]
        .or()
        .post()
        .expect("failed to post logical OR constraint");
    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");
    assert!(solution.get_bool_var(&bool1).unwrap() && solution.get_bool_var(&bool2).unwrap());
    assert!(solution.get_bool_var(&bool3).unwrap() || solution.get_bool_var(&bool4).unwrap());
}

#[test]
fn test_boolvar_if_then_else() {
    let model = Model::new(Some("TestBoolVarIfThenElse"));
    let b = model.bool_var(None, Some("b"));
    let x = IntVar::new(&model, (0, 10, Some("x")));
    let y = IntVar::new(&model, (0, 10, Some("y")));

    // Create constraints
    let then_constraint = x.eq(5);
    let else_constraint = y.eq(3);

    // Post if-then-else: if b is true, then x=5; else y=3
    b.if_then_else(&then_constraint, &else_constraint);

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("Expected a solution");

    // When b=true, x should be 5; when b=false, y should be 3
    let b_val = solution.get_bool_var(&b).unwrap();
    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();

    if b_val {
        assert_eq!(x_val, 5);
    } else {
        assert_eq!(y_val, 3);
    }
}

#[test]
fn test_boolvar_if_then() {
    let model = Model::new(Some("TestBoolVarIfThen"));
    let b = model.bool_var(None, Some("b"));
    let x = IntVar::new(&model, (0, 10, Some("x")));

    // Create constraint
    let then_constraint = x.eq(7);

    // Post if-then: if b is true, then x=7
    b.if_then(&then_constraint);

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("Expected a solution");

    // When b=true, x must be 7
    let b_val = solution.get_bool_var(&b).unwrap();
    let x_val = solution.get_int_var(&x).unwrap();

    if b_val {
        assert_eq!(x_val, 7);
    }
}

#[test]
fn test_boolvar_if_only_if() {
    let model = Model::new(Some("TestBoolVarIfOnlyIf"));
    let b = model.bool_var(None, Some("b"));
    let x = IntVar::new(&model, (0, 10, Some("x")));

    // Create constraint
    let constraint = x.eq(4);

    // Post if-only-if (equivalence): b is true <=> x=4
    b.if_only_if(&constraint);
    b.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("Expected a solution");

    // When b=true, x must be 4 (equivalence)
    let b_val = solution.get_bool_var(&b).unwrap();
    let x_val = solution.get_int_var(&x).unwrap();

    assert!(b_val);
    assert_eq!(x_val, 4);
}

#[test]
fn test_boolvar_bitwise_operators() {
    let model = Model::new(Some("TestBoolVarBitwiseOperators"));

    // Create boolean variables for testing
    let b1 = model.bool_var(None, Some("b1"));
    let b2 = model.bool_var(None, Some("b2"));
    let b3 = model.bool_var(None, Some("b3"));
    let b4 = model.bool_var(None, Some("b4"));

    // Test BitAnd: BoolVar & BoolVar (true & true = true)
    let and_result1 = &b1 & &b2;

    // Test BitAnd: BoolVar & bool (true & true = true)
    let and_result2 = &b1 & true;

    // Test BitAnd: bool & BoolVar (false & true = false)
    let and_result3 = false & &b2;

    // Test BitOr: BoolVar | BoolVar (false | true = true)
    let or_result1 = &b3 | &b4;

    // Test BitOr: BoolVar | bool (true | false = true)
    let or_result2 = &b1 | false;

    // Test BitOr: bool | BoolVar (true | false = true)
    let or_result3 = true | &b3;

    // Post constraints to force specific values
    b1.eq(true).post().unwrap();
    b2.eq(true).post().unwrap();
    b3.eq(false).post().unwrap();
    b4.eq(true).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Criterion::default())
        .expect("failed to find solution");

    // Verify input values
    let b1_val = solution.get_bool_var(&b1).unwrap();
    let b2_val = solution.get_bool_var(&b2).unwrap();
    let b3_val = solution.get_bool_var(&b3).unwrap();
    let b4_val = solution.get_bool_var(&b4).unwrap();

    assert!(b1_val);
    assert!(b2_val);
    assert!(!b3_val);
    assert!(b4_val);

    // Verify BitAnd results
    let and_result1_val = solution.get_bool_var(&and_result1).unwrap();
    let and_result2_val = solution.get_bool_var(&and_result2).unwrap();
    let and_result3_val = solution.get_bool_var(&and_result3).unwrap();

    assert!(and_result1_val); // true & true = true
    assert!(and_result2_val); // true & true = true
    assert!(!and_result3_val); // false & true = false

    // Verify BitOr results
    let or_result1_val = solution.get_bool_var(&or_result1).unwrap();
    let or_result2_val = solution.get_bool_var(&or_result2).unwrap();
    let or_result3_val = solution.get_bool_var(&or_result3).unwrap();

    assert!(or_result1_val); // false | true = true
    assert!(or_result2_val); // true | false = true
    assert!(or_result3_val); // true | false = true
}
