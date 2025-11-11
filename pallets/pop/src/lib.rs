#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
};
use frame_system::pallet_prelude::*;
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use frame_support::serde::{Deserialize, Serialize};

/// Minimum entropy threshold: 0.2 seconds of reaction time (20ms scaled)
const MIN_ENTROPY: u32 = 20;

/// Time-Token (TT) mint per valid PoP session (1 TT = 60 min)
type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PlaySession {
    pub player: Vec<u8>, // UGID hash
    pub entropy: u32,    // Reaction time in ms (scaled)
    pub timestamp: u64,
}

#[pallet::config]
pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type Currency: ReservableCurrency<Self::AccountId>;
}

#[pallet::pallet]
pub struct Pallet<T>(_);

#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    PlayVerified { player: T::AccountId, entropy: u32, tt_minted: BalanceOf<T> },
    InvalidPlay { reason: &'static str },
}

#[pallet::error]
pub enum Error<T> {
    LowEntropy,
    InvalidPlayer,
    Overflow,
}

#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(10_000)]
    pub fn submit_pop(
        origin: OriginFor<T>,
        player_ugid: Vec<u8>,
        entropy: u32,
    ) -> DispatchResult {
        let sender = ensure_signed(origin)?;

        ensure!(entropy >= MIN_ENTROPY, Error::<T>::LowEntropy);
        ensure!(!player_ugid.is_empty(), Error::<T>::InvalidPlayer);

        let tt_amount = BalanceOf::<T>::from(1u32);
        T::Currency::deposit_creating(&sender, tt_amount);

        Self::deposit_event(Event::PlayVerified {
            player: sender,
            entropy,
            tt_minted: tt_amount,
        });

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{assert_ok, assert_noop};
    use sp_core::H256;
    use sp_runtime::traits::BlakeTwo256;

    type BlockNumber = u32;
    type AccountId = u32;

    frame_support::construct_runtime!(
        pub enum Test where
            Block = frame_system::mocking::MockBlock<Test>,
            NativeCurrency = Balances,
            RuntimeCall = RuntimeCall,
            RuntimeEvent = RuntimeEvent,
        {
            System: frame_system,
            Balances: pallet_balances,
            PoP: super,
        }
    );

    #[test]
    fn valid_pop_mints_tt() {
        new_test_ext().execute_with(|| {
            let player = 1;
            assert_ok!(PoP::submit_pop(
                RuntimeOrigin::signed(player),
                b"ugid123".to_vec(),
                25
            ));
            assert_eq!(Balances::free_balance(player), 1);
        });
    }

    #[test]
    fn low_entropy_rejected() {
        new_test_ext().execute_with(|| {
            let player = 1;
            assert_noop!(
                PoP::submit_pop(RuntimeOrigin::signed(player), b"ugid".to_vec(), 10),
                Error::<Test>::LowEntropy
            );
        });
    }
}
