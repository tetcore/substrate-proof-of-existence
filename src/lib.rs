// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use srml_support::{decl_module, decl_storage, decl_event, ensure,
    StorageMap, dispatch::Result};
use srml_support::traits::{Currency, ReservableCurrency};
use sr_std::vec::Vec;
use system::ensure_signed;
use sr_primitives::traits::As;

const POE_FEE: u64 = 1000;

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: timestamp::Trait + balances::Trait {
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