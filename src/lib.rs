// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_module, decl_storage, decl_event, ensure,
    StorageMap, dispatch::Result};
use support::traits::{Currency, ReservableCurrency};
use rstd::vec::Vec;
use system::ensure_signed;
use runtime_primitives::traits::As;

const POE_FEE: u64 = 1000;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: timestamp::Trait {
    type Currency: ReservableCurrency<Self::AccountId>;
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as POEStorage {
		Proofs get(proofs): map Vec<u8> => (T::AccountId, T::Moment);
	}
}

decl_event!(
	pub enum Event<T> where
        <T as system::Trait>::AccountId,
        <T as timestamp::Trait>::Moment
    {
        ClaimCreated(AccountId, Moment, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        fn create_claim(origin, digest: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;

            ensure!(!<Proofs<T>>::exists(&digest), "This digest has already been claimed");
            let time = <timestamp::Module<T>>::now();

            T::Currency::reserve(&sender, BalanceOf::<T>::sa(POE_FEE))?;
            <Proofs<T>>::insert(&digest, (sender.clone(), time.clone()));

            Self::deposit_event(RawEvent::ClaimCreated(sender, time, digest));
            Ok(())
        }

        fn revoke_claim(origin, digest: Vec<u8>) -> Result {
            let sender = ensure_signed(origin)?;
            
            ensure!(<Proofs<T>>::exists(&digest), "This digest has not been claimed yet");
            let (owner, _time) = Self::proofs(&digest);

            ensure!(sender == owner, "You must own this claim to revoke it");

            <Proofs<T>>::remove(&digest);
            T::Currency::unreserve(&sender, BalanceOf::<T>::sa(POE_FEE));

            Self::deposit_event(RawEvent::ClaimRevoked(sender, digest));
            Ok(())
        }
    }
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, assert_noop};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl balances::Trait for Test {
		type Balance = u64;
		type OnNewAccount = ();
		type OnFreeBalanceZero = ();
		type Event = ();
		type TransactionPayment = ();
		type TransferPayment = ();
		type DustRemoval = ();
    }
    impl timestamp::Trait for Test {
        type Moment = u64;
        type OnTimestampSet = ();
    }
	impl Trait for Test {
		type Event = ();
        type Currency = balances::Module<Test>;
	}
    type Balances = balances::Module<Test>;
	type POEModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		let mut t = system::GenesisConfig::<Test>::default().build_storage().unwrap().0;
        t.extend(balances::GenesisConfig::<Test>{
			balances: vec![(1, 10000), (2, 10000)],
			transaction_base_fee: 0,
			transaction_byte_fee: 0,
			transfer_fee: 0,
			creation_fee: 0,
			existential_deposit: 0,
			vesting: vec![],
        }.build_storage().unwrap().0);
        t.into()

	}

	#[test]
	fn it_works() {
		with_externalities(&mut new_test_ext(), || {
			// Have account 1 create a claim
			assert_ok!(POEModule::create_claim(Origin::signed(1), vec![0]));
            // Check that account 1 reserved their deposit for creating a claim
            assert_eq!(Balances::free_balance(&1), 9000);
            assert_eq!(Balances::reserved_balance(&1), 1000);
            // Check that account 2 cannot create the same claim
            assert_noop!(POEModule::create_claim(Origin::signed(2), vec![0]), "This digest has already been claimed");
            // Check that account 2 cannot revoke a claim they do not own
            assert_noop!(POEModule::revoke_claim(Origin::signed(2), vec![0]), "You must own this claim to revoke it");
            // Check that account 2 cannot revoke some non-existent claim
            assert_noop!(POEModule::revoke_claim(Origin::signed(2), vec![1]), "This digest has not been claimed yet");
            // Check that account 1 can revoke their claim
            assert_ok!(POEModule::revoke_claim(Origin::signed(1), vec![0]));
            // Check that account 1 got back their deposit
            assert_eq!(Balances::free_balance(&1), 10000);
            assert_eq!(Balances::reserved_balance(&1), 0);
            // Check that account 2 can now claim this digest
            assert_ok!(POEModule::create_claim(Origin::signed(2), vec![0]));
		});
	}
}