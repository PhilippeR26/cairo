#[derive(Copy, Drop)]
extern type u248;

extern fn u248_try_from_felt252(value: felt252) -> Option<u248> implicits(RangeCheck) nopanic;
// extern fn u248_to_felt252(address: u248) -> felt252 nopanic;


