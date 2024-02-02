#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Olaylar için bir tür.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Balance başarıyla çekildi. Parametre olarak, hesap ID'si ve çekilen balance miktarı verilir.
        /// Balance withdrawn. Account ID and the amount of balance withdrawn are given as parameters.
        BalanceFetched(T::AccountId, T::Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
         /// Kullanıcının bu işlemi yapma yetkisi yok.
        Unauthorized,
        /// Belirtilen hesap bulunamadı.
        NotFound,
        /// Diğer hatalar.
        Other,
    }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn fetch_balance(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let _sender = ensure_signed(origin)?;

            let balance = <pallet_balances::Pallet<T>>::usable_balance(&account);

            
            Self::deposit_event(Event::BalanceFetched(account, balance));

            Ok(())
        }
    }
}
