#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchError},
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use pallet_balances::BalanceOf;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        // Pallet_balances kullanmak için eklenen tanım
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Balance başarıyla çekildi. Parametre olarak, hesap ID'si ve çekilen balance miktarı verilir.
        BalanceFetched(T::AccountId, BalanceOf<T>),
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

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
        pub fn fetch_balance(origin: OriginFor<T>, account: T::AccountId) -> DispatchResultWithPostInfo {
            let _sender = ensure_signed(origin).map_err(|_| Error::<T>::Unauthorized)?;

            // Balance'ı pallet_balances modülünden çekiyoruz.
            let balance = T::Currency::free_balance(&account);

            // Event'i tetikle
            Self::deposit_event(Event::BalanceFetched(account.clone(), balance));

            Ok(().into())
        }
    }
}
