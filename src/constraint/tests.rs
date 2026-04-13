use super::*;
use crate::*;

#[test]
fn test_arithm_intvar_equality_cst() {
    let model = Model::new(Some("TestArithmIntVarEqCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Eq, 5);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarEqCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_inequality_cst() {
    let model = Model::new(Some("TestArithmIntVarNeqCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Neq, 5);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarNeqCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_lessthan_cst() {
    let model = Model::new(Some("TestArithmIntVarLtCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Lt, 7);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarLtCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_leq_cst() {
    let model = Model::new(Some("TestArithmIntVarLeqCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Leq, 8);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarLeqCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_gt_cst() {
    let model = Model::new(Some("TestArithmIntVarGtCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Gt, 3);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarGtCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_geq_cst() {
    let model = Model::new(Some("TestArithmIntVarGeqCst"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    let constraint = var.arithm(EqualityOperator::Geq, 2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarGeqCst".to_string())
    );
}

#[test]
fn test_arithm_intvar_intvar_equality() {
    let model = Model::new(Some("TestArithmIntVarIntVarEq"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (5, 15, Some("var2")));

    let constraint = var1.arithm(EqualityOperator::Eq, &var2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarIntVarEq".to_string())
    );
}

#[test]
fn test_arithm_intvar_intvar_inequality() {
    let model = Model::new(Some("TestArithmIntVarIntVarNeq"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (5, 15, Some("var2")));

    let constraint = var1.arithm(EqualityOperator::Neq, &var2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarIntVarNeq".to_string())
    );
}

#[test]
fn test_arithm_intvar_intvar_lessthan() {
    let model = Model::new(Some("TestArithmIntVarIntVarLt"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (5, 15, Some("var2")));

    let constraint = var1.arithm(EqualityOperator::Lt, &var2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIntVarIntVarLt".to_string())
    );
}

#[test]
fn test_arithm_iv_op_iv_op_iv_with_intvar_generic() {
    println!("test_arithm_iv_op_iv_op_iv_with_intvar_generic");
    let model = Model::new(Some("TestArithmIvOpIvOpIvIntVar"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (2, 3, Some("var2")));
    let var3 = IntVar::new(&model, (5, 20, Some("var3")));

    // var1 = var2 + var3
    let constraint = var1.arithm2(EqualityOperator::Eq, &var2, ArithmeticOperator::Sum, &var3);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvOpIvOpIvIntVar".to_string())
    );
}

#[test]
fn test_arithm_iv_op_cst_op_iv() {
    println!("PATH: {:?}", std::env::var("PATH"));
    let model = Model::new(Some("TestArithmIvOpCstOpIv"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (5, 20, Some("var2")));

    // var1 = 3 + var2
    let constraint = var1.arithm2(EqualityOperator::Eq, 3, ArithmeticOperator::Sum, &var2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvOpCstOpIv".to_string())
    );
}

#[test]
fn test_arithm_iv_op_iv_op_cst_with_intvar_generic() {
    let model = Model::new(Some("TestArithmIvOpIvOpCstIntVar"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (2, 3, Some("var2")));

    // var1 + var2 = 8
    let constraint = var1.arithm2(EqualityOperator::Eq, &var2, ArithmeticOperator::Sum, 8);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvOpIvOpCstIntVar".to_string())
    );
}

#[test]
fn test_arithm_iv_op_cst_op_cst() {
    let model = Model::new(Some("TestArithmIvOpCstOpCst"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));

    // var1 + 2 = 5
    let constraint = var1.arithm2(EqualityOperator::Eq, 2, ArithmeticOperator::Sum, 5);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvOpCstOpCst".to_string())
    );
}

#[test]
fn test_arithm_iv_arithmop_iv_eqop_iv() {
    let model = Model::new(Some("TestArithmIvAopIvEopIv"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (2, 3, Some("var2")));
    let var3 = IntVar::new(&model, (5, 20, Some("var3")));

    // var1 + var2 = var3 (using reversed order)
    let constraint = var1.arithm2(ArithmeticOperator::Sum, &var2, EqualityOperator::Eq, &var3);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvAopIvEopIv".to_string())
    );
}

#[test]
fn test_arithm_iv_arithmop_cst_eqop_iv() {
    let model = Model::new(Some("TestArithmIvAopCstEopIv"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (5, 20, Some("var2")));

    // var1 + 3 = var2 (using reversed order)
    let constraint = var1.arithm2(ArithmeticOperator::Sum, 3, EqualityOperator::Eq, &var2);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvAopCstEopIv".to_string())
    );
}

#[test]
fn test_arithm_iv_arithmop_iv_eqop_cst() {
    let model = Model::new(Some("TestArithmIvAopIvEopCst"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (2, 3, Some("var2")));

    // var1 + var2 = 8 (using reversed order)
    let constraint = var1.arithm2(ArithmeticOperator::Sum, &var2, EqualityOperator::Eq, 8);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvAopIvEopCst".to_string())
    );
}

#[test]
fn test_arithm_iv_arithmop_cst_eqop_cst() {
    let model = Model::new(Some("TestArithmIvAopCstEopCst"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));

    // var1 + 2 = 5 (using reversed order)
    let constraint = var1.arithm2(ArithmeticOperator::Sum, 2, EqualityOperator::Eq, 5);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmIvAopCstEopCst".to_string())
    );
}

#[test]
fn test_arithm_subtraction() {
    let model = Model::new(Some("TestArithmSubtraction"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let var2 = IntVar::new(&model, (1, 5, Some("var2")));
    let var3 = IntVar::new(&model, (0, 10, Some("var3")));

    // var1 - var2 = var3
    let constraint = var1.arithm2(ArithmeticOperator::Sub, &var2, EqualityOperator::Eq, &var3);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmSubtraction".to_string())
    );
}

#[test]
fn test_arithm_multiplication() {
    let model = Model::new(Some("TestArithmMultiplication"));
    let var1 = IntVar::new(&model, (1, 5, Some("var1")));
    let var2 = IntVar::new(&model, (2, 3, Some("var2")));
    let var3 = IntVar::new(&model, (2, 15, Some("var3")));

    // var1 * var2 = var3
    let constraint = var1.arithm2(ArithmeticOperator::Mul, &var2, EqualityOperator::Eq, &var3);
    constraint.post().unwrap();
    assert_eq!(
        constraint.get_model().name(),
        Some("TestArithmMultiplication".to_string())
    );
}

#[test]
fn test_constraint_reify() {
    use crate::variables::Variable;

    let model = Model::new(Some("TestConstraintReify"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    // Create a constraint: var == 5
    let constraint = var.arithm(EqualityOperator::Eq, 5);

    // Reify the constraint
    let bool_var = constraint.reify();

    // Enforce the reified BoolVar to be true
    let reify_constraint = bool_var.eq(true);
    reify_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution when reified constraint is true");

    assert_eq!(solution.get_int_var(&var).unwrap(), 5);

    // The reified BoolVar should be valid
    assert!(bool_var.is_instantiated());
}

#[test]
fn test_constraint_reify_with() {
    use crate::variables::Variable;

    let model = Model::new(Some("TestConstraintReifyWith"));
    let var = IntVar::new(&model, (1, 10, Some("var")));
    let bool_var = BoolVar::new(&model, Some(true), Some("constraint_var"));

    // Create a constraint: var == 5
    let constraint = var.arithm(EqualityOperator::Eq, 5);

    // Reify the constraint with the given BoolVar
    constraint.reify_with(&bool_var);

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution when reify_with BoolVar is true");

    assert_eq!(solution.get_int_var(&var).unwrap(), 5);

    // No assertion needed, just verify it doesn't panic
    assert_eq!(bool_var.name(), "constraint_var");
}

#[test]
fn test_constraint_implies() {
    use crate::variables::Variable;

    let model = Model::new(Some("TestConstraintImplies"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let bool_var = BoolVar::new(&model, Some(true), Some("implication_var"));

    // Create a constraint: var1 == 5
    let constraint = var1.arithm(EqualityOperator::Eq, 5);

    // Set up implication: constraint implies bool_var
    constraint.implies(&bool_var);
    var1.arithm(EqualityOperator::Eq, 1).post().unwrap();
    // Enforce constraint to be true and bool_var to be false (infeasible)
    //sconstraint.post();
    let bool_constraint = bool_var.eq(true);
    bool_constraint.post().unwrap();

    let solver = model.solver();
    solver.propagate().expect("Propagation failed");
    let solution = solver.find_solution(&Default::default());
    if let Some(x) = solution {
        println!("{:?} {:?}", x.get_int_var(&var1), x.get_bool_var(&bool_var));
    } else {
        println!("No solution found, as expected due to implication constraint.");
    }

    // No assertion needed, just verify it doesn't panic
    assert_eq!(bool_var.name(), "implication_var");
}

#[test]
fn test_constraint_implied_by() {
    use crate::variables::Variable;

    let model = Model::new(Some("TestConstraintImpliedBy"));
    let var1 = IntVar::new(&model, (1, 10, Some("var1")));
    let bool_var = BoolVar::new(&model, None, Some("implication_var2"));

    // Create a constraint: var1 == 5
    let constraint = var1.arithm(EqualityOperator::Eq, 5);

    // Set up half-reification: constraint implied by bool_var
    constraint.implied_by(&bool_var);

    // Enforce bool_var to be true so constraint must hold
    let bool_constraint = bool_var.eq(true);
    bool_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver.find_solution(&Default::default());
    if let Some(x) = solution {
        println!("{:?} {:?}", x.get_int_var(&var1), x.get_bool_var(&bool_var));
    } else {
        println!("No solution found, as expected due to implication constraint.");
    }

    // No assertion needed, just verify it doesn't panic
    assert_eq!(bool_var.name(), "implication_var2");
}

#[test]
fn test_constraint_is_satisfied() {
    let model = Model::new(Some("TestConstraintIsSatisfied"));
    let var = IntVar::new(&model, (1, 10, Some("var")));

    // Create a constraint: var == 5
    let constraint = var.arithm(EqualityOperator::Eq, 5);
    constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution for var == 5");

    assert_eq!(solution.get_int_var(&var).unwrap(), 5);

    // Check the satisfaction state
    let state = constraint.is_satisfied();

    // The state should be one of the ESat variants
    assert_eq!(state, ESat::True);
}

#[test]
fn test_and_bool_vars_constraint() {
    let model = Model::new(Some("TestAndBoolVarsConstraint"));
    let b1 = BoolVar::new(&model, None, Some("b1"));
    let b2 = BoolVar::new(&model, None, Some("b2"));

    let and_constraint = [&b1, &b2].and();
    and_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution for AND of BoolVars");

    assert!(solution.get_bool_var(&b1).unwrap());
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_or_bool_vars_constraint() {
    let model = Model::new(Some("TestOrBoolVarsConstraint"));
    let b1 = BoolVar::new(&model, None, Some("b1"));
    let b2 = BoolVar::new(&model, None, Some("b2"));

    b1.eq(false).post().unwrap();

    let or_constraint = [&b1, &b2].or();
    or_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution for OR of BoolVars");

    assert!(!solution.get_bool_var(&b1).unwrap());
    assert!(solution.get_bool_var(&b2).unwrap());
}

#[test]
fn test_and_constraints() {
    let model = Model::new(Some("TestAndConstraints"));
    let x = IntVar::new(&model, (0, 10, Some("x")));

    let c1 = x.eq(5);
    let c2 = x.arithm(EqualityOperator::Geq, 5);

    let and_constraint = [&c1, &c2].and();
    and_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution for AND of constraints");

    assert_eq!(solution.get_int_var(&x).unwrap(), 5);
}

#[test]
fn test_or_constraints() {
    let model = Model::new(Some("TestOrConstraints"));
    let x = IntVar::new(&model, (0, 10, Some("x")));

    let c1 = x.eq(3);
    let c2 = x.eq(7);

    let or_constraint = [&c1, &c2].or();
    or_constraint.post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution for OR of constraints");

    let value = solution.get_int_var(&x).unwrap();
    assert!(value == 3 || value == 7);
}

#[test]
fn test_constraint_if_then_else() {
    let model = Model::new(Some("TestConstraintIfThenElse"));
    let x = IntVar::new(&model, (0, 10, Some("x")));
    let y = IntVar::new(&model, (0, 10, Some("y")));

    // Create constraints
    let if_constraint = x.eq(5);
    let then_constraint = y.eq(10);
    let else_constraint = y.eq(0);

    // Post if-then-else
    if_constraint.if_then_else(&then_constraint, &else_constraint);

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution");

    // When x=5, y should be 10; when x!=5, y should be 0
    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();

    if x_val == 5 {
        assert_eq!(y_val, 10);
    } else {
        assert_eq!(y_val, 0);
    }
}

#[test]
fn test_constraint_if_then() {
    let model = Model::new(Some("TestConstraintIfThen"));
    let x = IntVar::new(&model, (0, 10, Some("x")));
    let y = IntVar::new(&model, (0, 10, Some("y")));

    // Create constraints
    let if_constraint = x.eq(7);
    let then_constraint = y.eq(8);

    // Post if-then
    if_constraint.if_then(&then_constraint);

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution");

    // When x=7, y must be 8
    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();

    if x_val == 7 {
        assert_eq!(y_val, 8);
    }
}

#[test]
fn test_constraint_if_only_if() {
    let model = Model::new(Some("TestConstraintIfOnlyIf"));
    let x = IntVar::new(&model, (0, 10, Some("x")));
    let y = IntVar::new(&model, (0, 10, Some("y")));

    // Create constraints
    let constraint1 = x.eq(3);
    let constraint2 = y.eq(6);

    // Post if-only-if (equivalence)
    constraint1.if_only_if(&constraint2);
    x.eq(3).post().unwrap();

    let solver = model.solver();
    let solution = solver
        .find_solution(&Default::default())
        .expect("Expected a solution");

    // When x=3, y should also be 6 (equivalence)
    let x_val = solution.get_int_var(&x).unwrap();
    let y_val = solution.get_int_var(&y).unwrap();

    assert_eq!(x_val, 3);
    assert_eq!(y_val, 6);
}
