#[pallet::pallet]
pub mod weight_overflow_pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        // ...
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100)]
        pub fn do_something(origin: OriginFor<T>) -> DispatchResult {
            // ...
            Ok(())
        }

        #[pallet::weight(10)]
        pub fn do_something_else(origin: OriginFor<T>) -> DispatchResult {
            // ...
            Ok(())
        }

        #[pallet::weight(get_weight)]
        pub fn get_weight(origin: OriginFor<T>) -> DispatchResult {
            // ...
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn get_weight(function: &str) -> Weight {
            match function {
                "do_something" => 1000,
                "do_something_else" => 10,
                _ => 0,
            }
        }
    }
}
