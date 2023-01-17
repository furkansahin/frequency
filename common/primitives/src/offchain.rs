use codec::Decode;
use sp_io::{offchain, offchain_index};
use sp_runtime::offchain::{
	storage::{StorageRetrievalError, StorageValueRef},
	StorageKind,
};

/// Storage keys for offchain worker
/// THe tx_id is used to identify the transaction
const TX_ID: &[u8] = b"tx_id";
/// The lock is used to prevent the execution of the function
const DB_LOCK: &[u8] = b"lock";
/// The tx_id is used to identify the transaction in persistent storage
const DB_TX_ID: &[u8] = b"tx-id/";

/// Locks the execution of the function
#[derive(Debug)]
pub enum LockStatus {
	/// Lock is acquired
	Locked,
	/// Lock is released
	Release,
}

/// Acquires a lock on the execution of the function
pub fn lock<F>(prefix: &[u8], f: F) -> LockStatus
where
	F: Fn(),
{
	let lock_key = [prefix, DB_LOCK].concat();
	let mut lock_storage = StorageValueRef::persistent(&lock_key);

	let exec_id_opt = StorageValueRef::persistent(DB_TX_ID).get();
	if let Ok(Some(exec_id)) = exec_id_opt {
		let id_key = [prefix, TX_ID].concat();
		let id_storage = StorageValueRef::persistent(&id_key);
		let need_to_clear_lock =
			id_storage.mutate(|id: Result<Option<[u8; 32]>, StorageRetrievalError>| match id {
				Ok(Some(val)) => {
					if val != exec_id {
						// new id we need to clear lock because of first launch
						Ok(exec_id)
					} else {
						Err(())
					}
				},
				_ => {
					// no id we need to clear lock because of first launch
					Ok(exec_id)
				},
			});

		if need_to_clear_lock.is_ok() {
			lock_storage.clear();
		}
	}

	let can_process = lock_storage.mutate(
		|is_locked: Result<Option<bool>, StorageRetrievalError>| match is_locked {
			Ok(Some(true)) => Err(()),
			_ => Ok(true),
		},
	);

	match can_process {
		Ok(true) => {
			f();
			lock_storage.clear();
			LockStatus::Release
		},
		_ => LockStatus::Locked,
	}
}

/// Wrapper for offchain get operations
pub fn get_index_value<T: Decode>(
	kind: StorageKind,
	key: &[u8],
) -> Result<T, StorageRetrievalError> {
	match kind {
		StorageKind::PERSISTENT => {
			let indexed_value = get_impl::<T>(key);
			indexed_value
		},
		StorageKind::LOCAL => {
			let indexed_value = offchain::local_storage_get(kind, key);
			match indexed_value {
				Some(value) =>
					T::decode(&mut &value[..]).map_err(|_| StorageRetrievalError::Undecodable),
				None => Err(StorageRetrievalError::Undecodable),
			}
		},
	}
}

/// Sets a value by the key to offchain index
pub fn set_index_value(key: &[u8], value: &[u8]) {
	offchain_index::set(key, value);
}

/// Gets a value by the key from persistent storage
fn get_impl<T: Decode>(key: &[u8]) -> Result<T, StorageRetrievalError> {
	let oci_mem = StorageValueRef::persistent(&key);
	let val = oci_mem.get::<T>()?;
	match val {
		Some(value) => Ok(value),
		None => Err(StorageRetrievalError::Undecodable),
	}
}