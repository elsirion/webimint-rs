use anyhow::{anyhow, Result};
use fedimint_core::db::{
    IDatabase, IDatabaseTransaction, ISingleUseDatabaseTransaction, PrefixStream,
};
use fedimint_core::db::{IDatabaseTransactionOps, SingleUseDatabaseTransaction};
use fedimint_core::{apply, async_trait_maybe_send};
use rexie::{Direction, KeyRange, TransactionMode};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use tracing::{debug, info};

use futures::StreamExt;
use wasm_bindgen::JsValue;

const STORE_NAME: &str = "fedimint";

pub struct WasmDatabase(rexie::Rexie);

#[apply(async_trait_maybe_send!)]
impl IDatabase for WasmDatabase {
    async fn begin_transaction<'a>(&'a self) -> Box<dyn ISingleUseDatabaseTransaction<'a>> {
        let wasm_db_tx = WasmDatabaseTransaction(
            Some(
                self.0
                    .transaction(&[STORE_NAME], TransactionMode::Cleanup)
                    .expect("Could not start IndexedDB transaction"),
            ),
            PhantomData,
        );
        Box::new(SingleUseDatabaseTransaction::new(wasm_db_tx))
    }
}

impl Debug for WasmDatabase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WasmDatabase")
    }
}

pub struct WasmDatabaseTransaction<'a>(Option<rexie::Transaction>, PhantomData<&'a ()>);

#[apply(async_trait_maybe_send!)]
impl<'a> IDatabaseTransaction<'a> for WasmDatabaseTransaction<'a> {
    async fn commit_tx(mut self) -> anyhow::Result<()> {
        self.0
            .take()
            .ok_or(anyhow!("Transaction already committed"))?
            .commit()
            .await
            .map_err(|e| anyhow!("Could not commit IndexedDB transaction: {e:?}"))?;
        Ok(())
    }
}

#[apply(async_trait_maybe_send!)]
impl<'a> IDatabaseTransactionOps<'a> for WasmDatabaseTransaction<'a> {
    async fn raw_insert_bytes(
        &mut self,
        key: &[u8],
        value: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let old_value = self.raw_get_bytes(key).await?;

        let key = bytes_to_value(key);
        let value = bytes_to_value(value);
        self.0
            .as_ref()
            .ok_or(anyhow!("Transaction already committed"))?
            .store(STORE_NAME)
            .expect("Stroe exists")
            .put(&key, Some(&value))
            .await
            .map_err(|e| anyhow!("Could not insert entry into IndexedDB: {e:?}"))?;
        Ok(old_value)
    }

    async fn raw_get_bytes(&mut self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        let key = JsValue::from(hex::encode(key));
        let value = self
            .0
            .as_ref()
            .ok_or(anyhow!("Transaction already committed"))?
            .store(STORE_NAME)
            .expect("Store exists")
            .get(&key)
            .await
            .map_err(|e| anyhow!("Could fetch entry from IndexedDB: {e:?}"))?;
        if value.is_undefined() {
            Ok(None)
        } else {
            Ok(Some(
                hex::decode(value.as_string().expect("Value is a string"))
                    .map_err(|e| anyhow!("Failed to decode value from IndexedDB: {:?}", e))?,
            ))
        }
    }

    async fn raw_remove_entry(&mut self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        let old_value = self.raw_get_bytes(key).await?;

        let key = bytes_to_value(key);
        self.0
            .as_ref()
            .ok_or(anyhow!("Transaction already committed"))?
            .store(STORE_NAME)
            .expect("Store exists")
            .delete(&key)
            .await
            .map_err(|e| anyhow!("Could delete entry from IndexedDB: {e:?}"))?;
        Ok(old_value)
    }

    async fn raw_find_by_prefix(&mut self, key_prefix: &[u8]) -> anyhow::Result<PrefixStream<'_>> {
        // TODO: stream by limiting and using multiple queries
        let iter = fetch_prefix_items(
            self.0
                .as_ref()
                .ok_or(anyhow!("Transaction already committed"))?,
            key_prefix,
        )
        .await?;
        Ok(Box::pin(futures::stream::iter(iter)))
    }

    async fn raw_find_by_prefix_sorted_descending(
        &mut self,
        key_prefix: &[u8],
    ) -> anyhow::Result<PrefixStream<'_>> {
        let iter = fetch_prefix_items(
            self.0
                .as_ref()
                .ok_or(anyhow!("Transaction already committed"))?,
            key_prefix,
        )
        .await?
        .rev();
        Ok(Box::pin(futures::stream::iter(iter)))
    }

    async fn raw_remove_by_prefix(&mut self, key_prefix: &[u8]) -> Result<()> {
        let keys = self
            .raw_find_by_prefix(key_prefix)
            .await?
            .map(|kv| kv.0)
            .collect::<Vec<_>>()
            .await;
        for key in keys {
            self.raw_remove_entry(key.as_slice()).await?;
        }
        Ok(())
    }

    async fn rollback_tx_to_savepoint(&mut self) -> anyhow::Result<()> {
        unimplemented!("Savepoints are not supported in IndexedDB")
    }

    async fn set_tx_savepoint(&mut self) -> anyhow::Result<()> {
        unimplemented!("Savepoints are not supported in IndexedDB")
    }
}

impl<'a> Drop for WasmDatabaseTransaction<'a> {
    fn drop(&mut self) {
        if let Some(dbtx) = self.0.take() {
            info!("Aborting dbtx via Drop");
            wasm_bindgen_futures::spawn_local(async move {
                dbtx.abort().await.expect("Aborting DB transaction failed")
            });
        } else {
            debug!("Dropping already committed dbtx");
        }
    }
}

// When finding by prefix iterating in Reverse order, we need to start from
// "prefix+1" instead of "prefix", using lexicographic ordering. See the tests
// below.
// Will return None if there is no next prefix (i.e prefix is already the last
// possible/max one)
fn next_prefix(prefix: &[u8]) -> Option<Vec<u8>> {
    let mut next_prefix = prefix.to_vec();
    let mut is_last_prefix = true;
    for i in (0..next_prefix.len()).rev() {
        next_prefix[i] = next_prefix[i].wrapping_add(1);
        if next_prefix[i] > 0 {
            is_last_prefix = false;
            break;
        }
    }
    if is_last_prefix {
        // The given prefix is already the last/max prefix, so there is no next prefix,
        // return None to represent that
        None
    } else {
        Some(next_prefix)
    }
}

fn value_to_bytes(value: JsValue) -> Option<Vec<u8>> {
    if value.is_undefined() {
        None
    } else {
        Some(
            hex::decode(value.as_string().expect("Value is a string"))
                .expect("Value is a valid hex string"),
        )
    }
}

fn bytes_to_value(bytes: &[u8]) -> JsValue {
    JsValue::from(hex::encode(bytes))
}

async fn fetch_prefix_items(
    dbtx: &rexie::Transaction,
    key_prefix: &[u8],
) -> anyhow::Result<impl DoubleEndedIterator<Item = (Vec<u8>, Vec<u8>)>> {
    let first_key = bytes_to_value(key_prefix);
    let last_key = bytes_to_value(&next_prefix(key_prefix).unwrap_or(vec![]));

    let range = KeyRange::bound(&first_key, &last_key, false, true).expect("KeyRange is valid");
    let items = dbtx
        .store(STORE_NAME)
        .expect("Store exists")
        .get_all(Some(&range), None, None, Some(Direction::Next))
        .await
        .map_err(|e| anyhow!("Could not fetch items from IndexedDB: {e:?}"))?;

    Ok(items.into_iter().map(|(key, value)| {
        (
            value_to_bytes(key).expect("Entry exists"),
            value_to_bytes(value).expect("Entry exists"),
        )
    }))
}
