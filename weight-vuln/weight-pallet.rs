#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_module, decl_storage, decl_event, decl_error, dispatch,
    weights::{Weight, DispatchInfo},
};
use frame_system::ensure_signed;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_storage! {
    trait Store for Module<T: Config> as VulnerablePallet {
        // Bu pallet için özel storage item'ları burada tanımlanır.
        // Örneğin: Bir counter ya da başka bir state tutmak isteyebilirsiniz.
        // Value: u64;
    }
}

decl_event!(
    pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
        // Palletinizin tetiklediği event'ler burada tanımlanır.
        // Örneğin: Bir işlem tamamlandığında ya da bir hata oluştuğunda bir event tetikleyebilirsiniz.
        DummyEvent(AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        // Palletinizde meydana gelebilecek hataları burada tanımlayın.
        // Örneğin: Beklenmeyen bir değer, yetkisiz erişim, overflow/underflow vs.
        NoneValue,
        StorageOverflow,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        #[weight = 10_000]
        pub fn vulnerable_function(origin) -> dispatch::DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            // Zafiyetli weight hesaplama
            let actual_weight = Self::calculate_weight();
            
            // Burada, weight zafiyetinin simüle edildiği işlemler yer alır.
            // Örneğin, bu weight bilgisini kullanarak bazı işlemleri gerçekleştirebilirsiniz.
            // ...

            // Event yayınlama
            Self::deposit_event(RawEvent::DummyEvent(caller));

            // İşlem başarılı, gerçek weight ile sonuç döndür
            Ok(Some(actual_weight).into())
        }
    }
}

impl<T: Config> Module<T> {
    // Bu fonksiyon kasten hatalı bir weight hesaplama yapıyor olacak.
    fn calculate_weight() -> Weight {
        // Bu fonksiyonun içeriğini zafiyete göre özelleştirebilirsiniz.
        // Örneğin, bazı durumlarda çok düşük veya çok yüksek bir weight döndürebilir.
        1_000_000
    }
}
