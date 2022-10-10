//! Benchmarking setup for pallet-template

use crate::*;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks! {
	create_claim{
		let d in 0 .. T::MaxClaimLength::get();
		let claim = vec![0; d as usize];
		let caller : T::AccountId = whitelisted_caller();
	}:_(RawOrigin::Signed(caller.clone()), claim.clone())


	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}
