#![no_std]

use core::iter::{Iterator, ExactSizeIterator};

pub enum TransactionSourceError {
    TransactionNotFound,
    BundleNotFound,
}
pub enum TransactionStorageError {
    TransactionExists,
    Generic(&'static str),
}

pub struct Transaction;
pub struct TransactionView;
pub struct Hash;
pub struct Tag;

trait TransactionCursor
where
    Self: Iterator<Item = Transaction>
{
}
trait HashCursor
where
    Self: Iterator<Item = Hash>
{
}
trait BundleCursor
where
    Self: ExactSizeIterator<Item = Transaction>
{
}

trait ConfirmationTracker<HC: HashCursor> {
    fn bundle_is_confirmed(&self, bundle_hash: &Hash) -> Result<bool, TransactionSourceError>;
    fn tx_is_confirmed(&self, tx_hash: &Hash) -> Result<bool, TransactionSourceError>;

    fn unconfirmed_tx(&self) -> Result<HC, TransactionSourceError>;
    fn unconfirmed_bundles(&self) -> Result<HC, TransactionSourceError>;
}

trait ConfirmationStorage {
    fn mark_bundle_confirmed(&mut self, hash: &Hash) -> Result<(), TransactionStorageError>;
}

trait TransactionSource<TXC: TransactionCursor, HC: HashCursor, BXC: BundleCursor>
    
where
    Self: ConfirmationTracker<HC>,
{
    fn tx(&self, hash: &Hash) -> Result<Transaction, TransactionSourceError>;
    fn tx_balance(&self, tx_hash: &Hash) -> Result<usize, TransactionSourceError>;
    // can return an empty iterator
    fn tx_approovees(&self, hash: &Hash) -> Result<TXC, TransactionSourceError>;
    // can return BundleNotFound if we don't know the bundle yet.
    fn tx_bundle(&self, tx_hash: &Hash) -> Result<Hash, TransactionSourceError>;
    // can return an empty iterator
    fn tx_by_tag(&self, tag: &Tag) -> Result<TXC, TransactionSourceError>;
    fn tx_height(&self, tx_hash: &Hash) -> Result<usize, TransactionSourceError>;

    fn bundle_len(&self, bundle_hash: &Hash) -> Result<usize, TransactionSourceError>;
    fn bundle_tx(&self, bundle_hash: &Hash) -> Result<BXC, TransactionSourceError>;

    fn missing_tx(&self) -> Result<HC, TransactionSourceError>;
}


trait TransactionStorage
where
    Self: ConfirmationStorage,
{
    fn store(&mut self, tx: &TransactionView) -> Result<(), TransactionStorageError>;
    fn store_all(&mut self, tx: &[TransactionView]) -> Result<(), TransactionStorageError>;
}
