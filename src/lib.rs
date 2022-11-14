#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{DispatchResult, OptionQuery, *};
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[derive(Encode, Decode, Clone, PartialEq, TypeInfo, Default)]
    #[scale_info(skip_type_params(T))]
    pub struct Site<T: Config> {
        /// Domain name of the website.
        pub domain: Vec<u8>,
        /// Pinata SDK Link.
        pub file_url: Vec<u8>,
        /// Owner of the site (signer upon creation).
        pub owner: T::AccountId,
    }

    /// Mapping of domain names to sites
    #[pallet::storage]
    #[pallet::getter(fn site)]
    pub type DomainNamesToSites<T: Config> =
        StorageMap<_, Twox64Concat, Vec<u8>, Site<T>, OptionQuery>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/main-docs/build/events-errors/
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SiteCreated {
            domain: Vec<u8>,
        },
        SiteUpdated {
            domain: Vec<u8>,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
        pub fn create_new_site(
            origin: OriginFor<T>,
            domain: Vec<u8>,
            file_url: Vec<u8>,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let new_site = Site {
                domain: domain.clone(),
                file_url,
                owner,
            };
            <DomainNamesToSites<T>>::insert(domain.clone(), new_site);
            Self::deposit_event((Event::<T>::SiteCreated { domain }));
            Ok(())
        }
    }
}
