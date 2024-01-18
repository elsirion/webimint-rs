use std::collections::BTreeMap;

use fedimint_client::Client;
use fedimint_core::OutPoint;
use fedimint_prediction_markets_client::PredictionMarketsClientModule;
use fedimint_prediction_markets_common::{
    Candlestick, ContractOfOutcomeAmount, Market, Outcome, Seconds, UnixTimestamp,
};
use leptos::warn;
use secp256k1::PublicKey;
use tokio::sync::oneshot::{self};

use crate::client::{ClientRpc, RpcError, RpcRequest, RpcResponse};

pub mod components;
mod js;

#[derive(Debug, Clone)]
pub enum PredictionMarketsRpcRequest {
    GetPayoutControlPublicKey,
    GetMarket {
        market: OutPoint,
        from_local_cache: bool,
    },
    WaitCandlesticks {
        market: OutPoint,
        outcome: Outcome,
        candlestick_interval: Seconds,
        candlestick_timestamp: UnixTimestamp,
        candlestick_volume: ContractOfOutcomeAmount,
    },
}

pub enum PredictionMarketsRpcResponse {
    GetPayoutControlPublicKey(PublicKey),
    GetMarket(anyhow::Result<Option<Market>>),
    WaitCandlesticks(anyhow::Result<BTreeMap<UnixTimestamp, Candlestick>>),
}

impl PredictionMarketsRpcRequest {
    pub async fn handle(
        self,
        client: &Client,
        response_sender: oneshot::Sender<anyhow::Result<RpcResponse>>,
    ) {
        let prediction_markets_client = client.get_first_module::<PredictionMarketsClientModule>();

        match self {
            Self::GetPayoutControlPublicKey => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetPayoutControlPublicKey(
                            prediction_markets_client.get_client_payout_control(),
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetMarket {
                market,
                from_local_cache,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetMarket(
                            prediction_markets_client
                                .get_market(market, from_local_cache)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::WaitCandlesticks {
                market,
                outcome,
                candlestick_interval,
                candlestick_timestamp,
                candlestick_volume,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::WaitCandlesticks(
                            prediction_markets_client
                                .wait_candlesticks(
                                    market,
                                    outcome,
                                    candlestick_interval,
                                    candlestick_timestamp,
                                    candlestick_volume,
                                )
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
        }
    }
}

impl ClientRpc {
    pub async fn get_payout_control_public_key(&self) -> anyhow::Result<PublicKey, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::GetPayoutControlPublicKey,
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetPayoutControlPublicKey(pk),
            )) => Ok(pk),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_market(
        &self,
        market: OutPoint,
        from_local_cache: bool,
    ) -> anyhow::Result<Option<Market>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetMarket {
                    market,
                    from_local_cache,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetMarket(r))) => {
                r.map_err(|e| e.into())
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn wait_candlesticks(
        &self,
        market: OutPoint,
        outcome: Outcome,
        candlestick_interval: Seconds,
        candlestick_timestamp: UnixTimestamp,
        candlestick_volume: ContractOfOutcomeAmount,
    ) -> anyhow::Result<BTreeMap<UnixTimestamp, Candlestick>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::WaitCandlesticks {
                    market,
                    outcome,
                    candlestick_interval,
                    candlestick_timestamp,
                    candlestick_volume,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::WaitCandlesticks(r))) => {
                r.map_err(|e| e.into())
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }
}
