use test::test_utils::assert_eq;
use option::OptionTrait;

#[test]
fn test_u248_from_felt252() {
    let x = u248_try_from_felt252(1);
    assert(x.is_some(), '1 is not a u248');

    let pow_2_248 = 0x100000000000000000000000000000000000000000000000000000000000000;

    let y: Option<u248> = u248_try_from_felt252(pow_2_248);
    assert(y.is_none(), '2^248 is a u248');

    let z: Option<u248> = u248_try_from_felt252(pow_2_248 - 1);
    assert(z.is_some(), '2^248 - 1 is not a u248');
}
