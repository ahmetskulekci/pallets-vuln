#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    pallet_prelude::*,
    traits::{Currency, ReservableCurrency},
    weights::{DispatchClass, Pays, Weight},
};
use frame_system::pallet_prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> where AccountId = <T as frame_system::Config>::AccountId {
        DummyEvent(AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn vulnerable_function(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            // Zafiyetli weight hesaplama
            let actual_weight = Self::calculate_weight();

            // Burada, weight zafiyetinin simüle edildiği işlemler yer alır.
            // ...

            // Event yayınlama
            Self::deposit_event(Event::DummyEvent(caller));

            // İşlem başarılı, gerçek weight ile sonuç döndür
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        // Bu fonksiyon kasten hatalı bir weight hesaplama yapıyor olacak.
        fn calculate_weight() -> Weight {
            // Bu fonksiyonun içeriğini zafiyete göre özelleştirebilirsiniz.
            // Örneğin, bazı durumlarda çok düşük veya çok yüksek bir weight döndürebilir.
            1_000_000
        }
    }
}
