use std::collections::BTreeMap;
use std::fmt::Debug;
use std::sync::Arc;

use anyhow::Result;
use fedimint_core::db::mem_impl::{MemDatabase, MemTransaction};
use fedimint_core::db::{
    IDatabaseTransactionOps, IDatabaseTransactionOpsCore, IRawDatabase, IRawDatabaseTransaction,
    PrefixStream,
};
use fedimint_core::module::__reexports::serde_json;
use fedimint_core::{apply, async_trait_maybe_send};
use futures::StreamExt;
use gloo_storage::Storage;
use tracing::info;

#[derive(Clone, Debug)]
pub struct PersistentMemDb(Arc<MemDatabase>, String);

impl PersistentMemDb {
    pub async fn new(name: String) -> PersistentMemDb {
        let init_data: Vec<(Vec<u8>, Vec<u8>)> = match gloo_storage::LocalStorage::get(&name) {
            Ok(data) => data,
            Err(gloo_storage::errors::StorageError::KeyNotFound(_)) => Vec::new(),
            Err(e) => panic!("Error loading DB: {e}"),
        };

        let db = MemDatabase::new();
        {
            let mut dbtx = db.begin_transaction().await;

            for (key, value) in init_data {
                dbtx.raw_insert_bytes(&key, &value)
                    .await
                    .expect("inset failed");
            }

            dbtx.commit_tx()
                .await
                .expect("No dbtx running in parallel, can't fail");
        }

        PersistentMemDb(Arc::new(db), name)
    }

    pub fn list_dbs() -> Vec<String> {
        gloo_storage::LocalStorage::get_all::<BTreeMap<String, serde_json::Value>>()
            .unwrap()
            .into_keys()
            .collect()
    }
}

#[apply(async_trait_maybe_send!)]
impl IRawDatabase for PersistentMemDb {
    type Transaction<'a> = PersistentMemDbTransaction<'a>;

    async fn begin_transaction<'a>(&'a self) -> PersistentMemDbTransaction<'a> {
        PersistentMemDbTransaction(self.0.begin_transaction().await, self.1.clone())
    }
}

pub struct PersistentMemDbTransaction<'a>(MemTransaction<'a>, String);

#[apply(async_trait_maybe_send!)]
impl<'a> IDatabaseTransactionOpsCore for PersistentMemDbTransaction<'a> {
    async fn raw_insert_bytes(
        &mut self,
        key: &[u8],
        value: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        self.0.raw_insert_bytes(key, value).await
    }

    async fn raw_get_bytes(&mut self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        self.0.raw_get_bytes(key).await
    }

    async fn raw_remove_entry(&mut self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        self.0.raw_remove_entry(key).await
    }

    async fn raw_find_by_prefix(&mut self, key_prefix: &[u8]) -> anyhow::Result<PrefixStream<'_>> {
        self.0.raw_find_by_prefix(key_prefix).await
    }

    async fn raw_find_by_prefix_sorted_descending(
        &mut self,
        key_prefix: &[u8],
    ) -> anyhow::Result<PrefixStream<'_>> {
        self.0
            .raw_find_by_prefix_sorted_descending(key_prefix)
            .await
    }

    async fn raw_remove_by_prefix(&mut self, key_prefix: &[u8]) -> Result<()> {
        self.0.raw_remove_by_prefix(key_prefix).await
    }
}

#[apply(async_trait_maybe_send!)]
impl<'a> IDatabaseTransactionOps for PersistentMemDbTransaction<'a> {
    async fn set_tx_savepoint(&mut self) -> anyhow::Result<()> {
        self.0.set_tx_savepoint().await
    }

    async fn rollback_tx_to_savepoint(&mut self) -> anyhow::Result<()> {
        self.0.rollback_tx_to_savepoint().await
    }
}

#[apply(async_trait_maybe_send!)]
impl<'a> IRawDatabaseTransaction for PersistentMemDbTransaction<'a> {
    async fn commit_tx(mut self) -> Result<()> {
        let dump = self
            .0
            .raw_find_by_prefix(&[])
            .await
            .expect("Dumping DB failed")
            .collect::<Vec<(Vec<u8>, Vec<u8>)>>()
            .await;
        self.0.commit_tx().await?;

        info!("Writing DB dump of {} kv pairs", dump.len());

        // FIXME: more compact format
        gloo_storage::LocalStorage::set(&self.1, dump).expect("Could not store DB");

        Ok(())
    }
}
