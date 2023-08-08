use fedimint_client::secret::PlainRootSecretStrategy;
use fedimint_core::api::InviteCode;
use fedimint_core::db::mem_impl::MemDatabase;
use fedimint_core::task::TaskGroup;
use fedimint_ln_client::LightningClientGen;
use fedimint_mint_client::MintClientGen;
use fedimint_wallet_client::WalletClientGen;
use std::str::FromStr;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, info};

#[derive(Debug, Clone)]
enum RpcRequest {
    Join(String),
    GetName,
}

#[derive(Debug, Clone)]
enum RpcResponse {
    Join,
    GetName(String),
}

type RpcCall = (RpcRequest, oneshot::Sender<anyhow::Result<RpcResponse>>);

async fn run_client(mut rpc: mpsc::Receiver<RpcCall>) {
    let client = loop {
        let (invite_code_str, response_sender) = match rpc.recv().await.expect("Sender not dropped")
        {
            (RpcRequest::Join(invite_code_str), response_sender) => {
                (invite_code_str, response_sender)
            }
            (_, response_sender) => {
                response_sender
                    .send(Err(anyhow::anyhow!(
                        "Invalid request, need to initialize client first"
                    )))
                    .unwrap();
                continue;
            }
        };

        let invite_code = match InviteCode::from_str(&invite_code_str) {
            Ok(invite) => invite,
            Err(e) => {
                response_sender
                    .send(Err(anyhow::anyhow!("Invalid invite code: {e:?}")))
                    .unwrap();
                continue;
            }
        };

        let mut client_builder = fedimint_client::Client::builder();
        client_builder.with_module(WalletClientGen(None));
        client_builder.with_module(MintClientGen);
        client_builder.with_module(LightningClientGen);
        client_builder.with_database(MemDatabase::new());
        client_builder.with_primary_module(1);
        client_builder.with_invite_code(invite_code);
        let tg = TaskGroup::new();
        let client_res = client_builder.build::<PlainRootSecretStrategy>(tg).await;

        match client_res {
            Ok(client) => {
                response_sender
                    .send(Ok(RpcResponse::Join))
                    .expect("RPC receiver not dropped");
                break client;
            }
            Err(e) => {
                response_sender
                    .send(Err(anyhow::anyhow!("Failed to initialize client: {e:?}")))
                    .unwrap();
                continue;
            }
        };
    };

    while let Some((rpc_request, response_sender)) = rpc.recv().await {
        debug!("Received RPC request: {:?}", rpc_request);
        match rpc_request {
            RpcRequest::GetName => {
                let name = client
                    .get_meta("federation_name")
                    .unwrap_or("<unknown>".to_string());
                response_sender
                    .send(Ok(RpcResponse::GetName(name)))
                    .unwrap();
            }
            req => response_sender
                .send(Err(anyhow::anyhow!("Invalid request: {req:?}")))
                .unwrap(),
        }
    }

    info!("Client RPC handler shutting down");
}

#[derive(Clone)]
pub struct ClientRpc {
    sender: mpsc::Sender<RpcCall>,
}

impl ClientRpc {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(10);
        wasm_bindgen_futures::spawn_local(run_client(receiver));
        Self { sender }
    }

    pub async fn join(&self, invite: String) -> anyhow::Result<()> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::Join(invite), response_sender))
            .await?;
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::Join => Ok(()),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }

    pub async fn get_name(&self) -> anyhow::Result<String> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::GetName, response_sender))
            .await?;
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::GetName(name) => Ok(name),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }
}
