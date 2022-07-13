#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;

#[frame_support::pallet]
pub mod pallet {
	pub use super::*;
	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T:Config> {
		dna: Vec<u8>,
		price:u32,
		gender: Gender,
		owner: T::AccountId,
	}
	pub type Id = u32;

	#[derive(TypeInfo, Encode ,Decode, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default()-> Self{
			Gender::Male
		}
	}
	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type KittyId<T> = StorageValue<_, Id,ValueQuery>;


	// key : id
	//value : student
	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitties<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn balance)]
	pub type Balance<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Vec<u32>, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn kitty_info)]
	// pub type KittyInfo<T: Config> = StorageMap<_, Blake2_128, Id, KittyInfo<T>, OptionQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn tottal_kitty)]
	// pub type TotalKitty<T: Config> = StorageValue<_, u32>;
	// key : id owner
	//value : owner
	#[pallet::storage]
	#[pallet::getter(fn ownerkitty)]
	pub(super) type Owner<T: Config> = StorageMap<_, Blake2_128Concat,T::AccountId , Vec<u8>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T:Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStored(Vec<u8>,u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		TooCheap,
		AlreadyExists,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		NotExits,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.

	//extrinsic
	#[pallet::call]
	impl<T:Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>,dna: Vec<u8>, price: u32) -> DispatchResult {
			
			let who = ensure_signed(origin)?;
			ensure!(price > 100, Error::<T>::TooCheap);
			let gender = Self::gen_gender(dna.clone())?;
			let kitty = Kitty {
				dna: dna.clone(),
				price: price,
				gender: gender,
				owner: who.clone(),
			};
			let mut current_id = <KittyId<T>>::get();
			<Kitties<T>>::insert(current_id, kitty);
			current_id +=1;
			KittyId::<T>::put(current_id);
			<Owner<T>>::insert(who.clone(), dna.clone());
			Self::deposit_event(Event::KittyStored(dna.clone(),price));
			
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn change_owner(origin: OriginFor<T>,kitty_id: u32, new_owner: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			//let kitty_id = KittyId::<T>::get();
			let kitty = <Kitties<T>>::get(kitty_id).unwrap();
			ensure!(kitty.owner == sender, Error::<T>::AlreadyExists);
			let new_kitty = Kitty {
				dna: kitty.dna,
				price: kitty.price,
				gender: kitty.gender,
				owner: new_owner,
			};
			<Kitties<T>>::insert(kitty_id, new_kitty);
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn update_price(origin: OriginFor<T>,kitty_id: u32, new_price: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
		
			//let id = KittyId::<T>::get();
			//ensure!(!id == kitty_id, Error::<T>::NotExits);
			let kitty = <Kitties<T>>::get(kitty_id.clone()).unwrap();
			ensure!(who == kitty.owner, Error::<T>::NotExits);
			let new_kitty = Kitty{
				dna: kitty.dna,
				price: new_price,
				gender: kitty.gender,
				owner: kitty.owner,
			};
			//<Kitties<T>>::remove(kitty_id.clone());
			<Kitties<T>>::insert(kitty_id.clone(), new_kitty);
			Ok(())
		}
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn get_owner_kitty(origin: OriginFor<T>,owner: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let kitty = <Owner<T>>::get(owner.clone());
			Ok(())
		}
		
	}
}

// helper function

impl<T> Pallet<T> {
	fn gen_gender(dna: Vec<u8>) -> Result<Gender,Error<T>>{
		let mut res = Gender::Male;
		if dna.len() % 2 ==0 {
			res = Gender::Female;
		}
		else{
			res= Gender::Male;
		}
		Ok(res)
	}

}

