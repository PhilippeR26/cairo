use cairo_felt::Felt252;
use cairo_lang_casm::builder::CasmBuilder;
use cairo_lang_casm::casm_build_extend;
use cairo_lang_sierra::extensions::u248::U248ConcreteLibfunc;
use num_bigint::{BigInt, ToBigInt};

use super::{CompiledInvocation, CompiledInvocationBuilder, InvocationError};
use crate::invocations::misc::validate_under_limit;
use crate::invocations::{
    add_input_variables, get_non_fallthrough_statement_id, CostValidationInfo,
};

// TODO(yg): can we do it as const?
fn get_u248_limit() -> BigInt {
    let x: BigInt = BigInt::from(u128::MAX) + 1;
    x.pow(2)
}
// TODO(yg): needed?
fn get_u248_max() -> BigInt {
    get_u248_limit() - 1
}

/// Builds instructions for Sierra u248 operations.
pub fn build(
    libfunc: &U248ConcreteLibfunc,
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    match libfunc {
        U248ConcreteLibfunc::TryFromFelt252(_) => build_u248_try_from_felt252(builder),
        // TODO(yg): more
    }
}

// TODO(yg): copied from build_u251_try_from_felt252. Share code?
// TODO(yg): review
/// builds a libfunc that tries to convert a felt252 to type with values in the range[0, 2**248).
pub fn build_u248_try_from_felt252(
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    let value_bound: BigInt = BigInt::from(1) << 248;
    let [range_check, value] = builder.try_get_single_cells()?;
    let failure_handle_statement_id = get_non_fallthrough_statement_id(&builder);
    let mut casm_builder = CasmBuilder::default();
    add_input_variables! {casm_builder,
        buffer(2) range_check;
        deref value;
    };
    let auxiliary_vars: [_; 4] = std::array::from_fn(|_| casm_builder.alloc_var(false));
    casm_build_extend! {casm_builder,
        const limit = value_bound.clone();
        let orig_range_check = range_check;
        tempvar is_valid_value;
        hint TestLessThan {lhs: value, rhs: limit} into {dst: is_valid_value};
        jump IsValidValue if is_valid_value != 0;
        tempvar shifted_value = value - limit;
    }
    validate_under_limit::<1>(
        &mut casm_builder,
        &(Felt252::prime().to_bigint().unwrap() - value_bound.clone()),
        shifted_value,
        range_check,
        &auxiliary_vars,
    );
    casm_build_extend! {casm_builder,
        jump Failure;
        IsValidValue:
    };
    validate_under_limit::<1>(&mut casm_builder, &value_bound, value, range_check, &auxiliary_vars);
    Ok(builder.build_from_casm_builder(
        casm_builder,
        [
            ("Fallthrough", &[&[range_check], &[value]], None),
            ("Failure", &[&[range_check]], Some(failure_handle_statement_id)),
        ],
        CostValidationInfo {
            range_check_info: Some((orig_range_check, range_check)),
            extra_costs: None,
        },
    ))
}
