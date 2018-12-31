use srml_support::{StorageMap, dispatch::Result};
use rstd::vec::Vec;
use system::ensure_signed;
use runtime_primitives::traits::As;

const POE_FEE: u64 = 1000;

pub trait Trait: timestamp::Trait + balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        fn create_claim(origin, digest: Vec<u8>) -> Result {
            ensure!(!<Proofs<T>>::exists(&digest), "This digest has already been claimed");

            let sender = ensure_signed(origin)?;
            let time = <timestamp::Module<T>>::now();

            <balances::Module<T>>::decrease_free_balance(&sender, <T::Balance as As<u64>>::sa(POE_FEE))?;
            <Proofs<T>>::insert(&digest, (sender.clone(), time.clone()));

            Self::deposit_event(RawEvent::ClaimCreated(sender, time, digest));
            Ok(())
        }

        fn revoke_claim(origin, digest: Vec<u8>) -> Result {
            ensure!(<Proofs<T>>::exists(&digest), "This digest has not been claimed yet");

            let sender = ensure_signed(origin)?;
            let (owner, _time) = Self::proofs(&digest);

            ensure!(sender == owner, "You must own this claim to revoke it");

            <Proofs<T>>::remove(&digest);
            <balances::Module<T>>::increase_free_balance_creating(&sender, <T::Balance as As<u64>>::sa(POE_FEE));

            Self::deposit_event(RawEvent::ClaimRevoked(sender, digest));
            Ok(())
        }
    }
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
