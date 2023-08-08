use fedimint_client::secret::PlainRootSecretStrategy;
use fedimint_client::Client;
use fedimint_core::api::InviteCode;
use fedimint_core::db::mem_impl::MemDatabase;
use fedimint_core::task::TaskGroup;
use fedimint_core::util::BoxStream;
use fedimint_core::Amount;
use fedimint_ln_client::LightningClientGen;
use fedimint_mint_client::MintClientGen;
use fedimint_mint_client::{parse_ecash, MintClientExt};
use fedimint_wallet_client::WalletClientGen;
use leptos::warn;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, info};

#[derive(Debug, Clone)]
enum RpcRequest {
    Join(String),
    GetName,
    SubscribeBalance,
    Receive(String),
}

enum RpcResponse {
    Join,
    GetName(String),
    SubscribeBalance(BoxStream<'static, Amount>),
    Receive(Amount),
}

impl Debug for RpcResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RpcResponse::?")
    }
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
                let _ = response_sender
                    .send(Err(anyhow::anyhow!(
                        "Invalid request, need to initialize client first"
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                continue;
            }
        };

        info!("Joining federation {}", invite_code_str);

        let invite_code = match InviteCode::from_str(&invite_code_str) {
            Ok(invite) => invite,
            Err(e) => {
                let _ = response_sender
                    .send(Err(anyhow::anyhow!("Invalid invite code: {e:?}")))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
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
                let _ = response_sender
                    .send(Ok(RpcResponse::Join))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                break client;
            }
            Err(e) => {
                let _ = response_sender
                    .send(Err(anyhow::anyhow!("Failed to initialize client: {e:?}")))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
                continue;
            }
        };
    };

    let client: &Client = Box::leak(Box::new(client));

    while let Some((rpc_request, response_sender)) = rpc.recv().await {
        debug!("Received RPC request: {:?}", rpc_request);
        match rpc_request {
            RpcRequest::GetName => {
                let name = client
                    .get_meta("federation_name")
                    .unwrap_or("<unknown>".to_string());
                let _ = response_sender
                    .send(Ok(RpcResponse::GetName(name)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::SubscribeBalance => {
                let stream = client.subscribe_balance_changes().await;
                let _ = response_sender
                    .send(Ok(RpcResponse::SubscribeBalance(stream)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            RpcRequest::Receive(notes) => {
                let notes = notes.trim();
                info!("Receiving notes: \"{notes}\"");
                let notes = parse_ecash(notes).unwrap();
                let amount = notes.total_amount();
                client.reissue_external_notes(notes, ()).await.unwrap();
                let _ = response_sender
                    .send(Ok(RpcResponse::Receive(amount)))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            req => {
                let _ = response_sender
                    .send(Err(anyhow::anyhow!("Invalid request: {req:?}")))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
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
            .await
            .expect("Client has stopped");
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
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::GetName(name) => Ok(name),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }

    pub async fn subscribe_balance(&self) -> anyhow::Result<BoxStream<'static, Amount>> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::SubscribeBalance, response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::SubscribeBalance(stream) => Ok(stream),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }

    pub async fn receive(&self, invoice: String) -> anyhow::Result<Amount> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((RpcRequest::Receive(invoice), response_sender))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped")?;
        match response {
            RpcResponse::Receive(amount) => Ok(amount),
            _ => Err(anyhow::anyhow!("Invalid response")),
        }
    }
}
