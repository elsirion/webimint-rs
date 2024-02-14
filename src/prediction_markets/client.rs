use std::collections::{BTreeMap, HashMap};

use fedimint_client::Client;
use fedimint_core::{Amount, OutPoint};
use fedimint_prediction_markets_client::PredictionMarketsClientModule;
use fedimint_prediction_markets_common::config::GeneralConsensus;
use fedimint_prediction_markets_common::{
    Candlestick, ContractOfOutcomeAmount, Market, MarketInformation, Order, OrderIdClientSide,
    Outcome, Seconds, Side, UnixTimestamp, Weight, WeightRequiredForPayout,
};
use tracing::warn;
use secp256k1::PublicKey;
use serde::Serialize;
use tokio::sync::oneshot::{self};

use crate::client::{ClientRpc, RpcError, RpcRequest, RpcResponse};

#[derive(Debug, Clone, Serialize)]
pub enum PredictionMarketsRpcRequest {
    GetGeneralConsensus,
    GetClientPayoutControl,
    NewMarket {
        contract_price: Amount,
        outcomes: Outcome,
        payout_control_weights: BTreeMap<PublicKey, Weight>,
        weight_required_for_payout: WeightRequiredForPayout,
        payout_controls_fee_per_contract: Amount,
        information: MarketInformation,
    },
    GetMarket {
        market: OutPoint,
        from_local_cache: bool,
    },
    ProposePayout {
        market_out_point: OutPoint,
        outcome_payouts: Vec<Amount>,
    },
    GetMarketPayoutControlProposals {
        market: OutPoint,
        from_local_cache: bool,
    },
    GetClientPayoutControlMarkets {
        from_local_cache: bool,
        markets_created_after_and_including: UnixTimestamp,
    },
    SendPayoutControlBitcoinBalanceToPrimaryModule,
    NewOrder {
        market: OutPoint,
        outcome: Outcome,
        side: Side,
        price: Amount,
        quantity: ContractOfOutcomeAmount,
    },
    GetOrder {
        id: OrderIdClientSide,
        from_local_cache: bool,
    },
    CancelOrder {
        id: OrderIdClientSide,
    },
    SendOrderBitcoinBalanceToPrimaryModule,
    SyncOrders {
        sync_possible_payouts: bool,
        market: Option<OutPoint>,
        outcome: Option<Outcome>,
    },
    GetOrdersFromDb {
        market: Option<OutPoint>,
        outcome: Option<Outcome>,
    },
    ResyncOrderSlots {
        gap_size_to_check: u16,
    },
    GetCandlesticks {
        market: OutPoint,
        outcome: Outcome,
        candlestick_interval: Seconds,
        min_candlestick_timestamp: UnixTimestamp,
    },
    WaitCandlesticks {
        market: OutPoint,
        outcome: Outcome,
        candlestick_interval: Seconds,
        candlestick_timestamp: UnixTimestamp,
        candlestick_volume: ContractOfOutcomeAmount,
    },
    SaveMarket {
        market: OutPoint,
    },
    UnsaveMarket {
        market: OutPoint,
    },
    GetSavedMarkets,
    SetNameToPayoutControl {
        name: String,
        payout_control: Option<PublicKey>,
    },
    GetNameToPayoutControl {
        name: String,
    },
    GetNameToPayoutControlMap,
}

#[derive(Debug)]
pub enum PredictionMarketsRpcResponse {
    GetGeneralConsensus(GeneralConsensus),
    GetClientPayoutControl(PublicKey),
    NewMarket(anyhow::Result<OutPoint>),
    GetMarket(anyhow::Result<Option<Market>>),
    ProposePayout(anyhow::Result<()>),
    GetMarketPayoutControlProposals(anyhow::Result<BTreeMap<PublicKey, Vec<Amount>>>),
    GetClientPayoutControlMarkets(anyhow::Result<BTreeMap<UnixTimestamp, Vec<OutPoint>>>),
    SendPayoutControlBitcoinBalanceToPrimaryModule(anyhow::Result<Amount>),
    NewOrder(anyhow::Result<OrderIdClientSide>),
    GetOrder(anyhow::Result<Option<Order>>),
    CancelOrder(anyhow::Result<()>),
    SendOrderBitcoinBalanceToPrimaryModule(anyhow::Result<Amount>),
    SyncOrders(anyhow::Result<BTreeMap<OrderIdClientSide, Order>>),
    GetOrdersFromDb(BTreeMap<OrderIdClientSide, Order>),
    ResyncOrderSlots(anyhow::Result<()>),
    GetCandlesticks(anyhow::Result<BTreeMap<UnixTimestamp, Candlestick>>),
    WaitCandlesticks(anyhow::Result<BTreeMap<UnixTimestamp, Candlestick>>),
    SaveMarket,
    UnsaveMarket,
    GetSavedMarkets(Vec<(OutPoint, UnixTimestamp)>),
    SetNameToPayoutControl,
    GetNameToPayoutControl(Option<PublicKey>),
    GetNameToPayoutControlMap(HashMap<String, PublicKey>),
}

impl PredictionMarketsRpcRequest {
    pub async fn handle(
        self,
        client: &Client,
        response_sender: oneshot::Sender<anyhow::Result<RpcResponse>>,
    ) {
        let prediction_markets_client = client.get_first_module::<PredictionMarketsClientModule>();

        match self {
            Self::GetGeneralConsensus => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetGeneralConsensus(
                            prediction_markets_client.get_general_consensus(),
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetClientPayoutControl => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetClientPayoutControl(
                            prediction_markets_client.get_client_payout_control(),
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::NewMarket {
                contract_price,
                outcomes,
                payout_control_weights,
                weight_required_for_payout,
                payout_controls_fee_per_contract,
                information,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::NewMarket(
                            prediction_markets_client
                                .new_market(
                                    contract_price,
                                    outcomes,
                                    payout_control_weights,
                                    weight_required_for_payout,
                                    payout_controls_fee_per_contract,
                                    information,
                                )
                                .await,
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
            Self::ProposePayout {
                market_out_point,
                outcome_payouts,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::ProposePayout(
                            prediction_markets_client
                                .propose_payout(market_out_point, outcome_payouts)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetMarketPayoutControlProposals {
                market,
                from_local_cache,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetMarketPayoutControlProposals(
                            prediction_markets_client
                                .get_market_payout_control_proposals(market, from_local_cache)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetClientPayoutControlMarkets {
                from_local_cache,
                markets_created_after_and_including,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetClientPayoutControlMarkets(
                            prediction_markets_client
                                .get_client_payout_control_markets(
                                    from_local_cache,
                                    markets_created_after_and_including,
                                )
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }

            Self::SendPayoutControlBitcoinBalanceToPrimaryModule => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::SendPayoutControlBitcoinBalanceToPrimaryModule(
                            prediction_markets_client
                                .send_payout_control_bitcoin_balance_to_primary_module()
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::NewOrder {
                market,
                outcome,
                side,
                price,
                quantity,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::NewOrder(
                            prediction_markets_client
                                .new_order(market, outcome, side, price, quantity)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetOrder {
                id,
                from_local_cache,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetOrder(
                            prediction_markets_client
                                .get_order(id, from_local_cache)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::CancelOrder { id } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::CancelOrder(
                            prediction_markets_client.cancel_order(id).await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::SendOrderBitcoinBalanceToPrimaryModule => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::SendOrderBitcoinBalanceToPrimaryModule(
                            prediction_markets_client
                                .send_order_bitcoin_balance_to_primary_module()
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::SyncOrders {
                sync_possible_payouts,
                market,
                outcome,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::SyncOrders(
                            prediction_markets_client
                                .sync_orders(sync_possible_payouts, market, outcome)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetOrdersFromDb { market, outcome } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetOrdersFromDb(
                            prediction_markets_client
                                .get_orders_from_db(market, outcome)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::ResyncOrderSlots { gap_size_to_check } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::ResyncOrderSlots(
                            prediction_markets_client
                                .resync_order_slots(gap_size_to_check)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetCandlesticks {
                market,
                outcome,
                candlestick_interval,
                min_candlestick_timestamp,
            } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetCandlesticks(
                            prediction_markets_client
                                .get_candlesticks(
                                    market,
                                    outcome,
                                    candlestick_interval,
                                    min_candlestick_timestamp,
                                )
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
            Self::SaveMarket { market } => {
                prediction_markets_client.save_market(market).await;

                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::SaveMarket,
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::UnsaveMarket { market } => {
                prediction_markets_client.unsave_market(market).await;

                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::UnsaveMarket,
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetSavedMarkets => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetSavedMarkets(
                            prediction_markets_client.get_saved_markets().await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::SetNameToPayoutControl {
                name,
                payout_control,
            } => {
                prediction_markets_client
                    .set_name_to_payout_control(name, payout_control)
                    .await;

                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::SetNameToPayoutControl,
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetNameToPayoutControl { name } => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetNameToPayoutControl(
                            prediction_markets_client
                                .get_name_to_payout_control(name)
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
            Self::GetNameToPayoutControlMap => {
                _ = response_sender
                    .send(Ok(RpcResponse::PredictionMarkets(
                        PredictionMarketsRpcResponse::GetNameToPayoutControlMap(
                            prediction_markets_client
                                .get_name_to_payout_control_map()
                                .await,
                        ),
                    )))
                    .map_err(|_| warn!("RPC receiver dropped before response was sent"));
            }
        }
    }
}

impl ClientRpc {
    pub async fn get_general_consensus(&self) -> anyhow::Result<GeneralConsensus, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetGeneralConsensus),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetGeneralConsensus(gc),
            )) => Ok(gc),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_client_payout_control(&self) -> anyhow::Result<PublicKey, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetClientPayoutControl),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetClientPayoutControl(pk),
            )) => Ok(pk),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn new_market(
        &self,
        contract_price: Amount,
        outcomes: Outcome,
        payout_control_weights: BTreeMap<PublicKey, Weight>,
        weight_required_for_payout: WeightRequiredForPayout,
        payout_controls_fee_per_contract: Amount,
        information: MarketInformation,
    ) -> anyhow::Result<OutPoint, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::NewMarket {
                    contract_price,
                    outcomes,
                    payout_control_weights,
                    weight_required_for_payout,
                    payout_controls_fee_per_contract,
                    information,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::NewMarket(Ok(r)))) => {
                Ok(r)
            }
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::NewMarket(Err(e)))) => {
                Err(RpcError::ClientStopped(e.to_string()))
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_market(
        &self,
        market_out_point: OutPoint,
        from_local_cache: bool,
    ) -> anyhow::Result<Option<Market>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetMarket {
                    market: market_out_point,
                    from_local_cache,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetMarket(Ok(r)))) => {
                Ok(r)
            }
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetMarket(Err(e)))) => {
                Err(RpcError::ClientStopped(e.to_string()))
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn propose_payout(
        &self,
        market_out_point: OutPoint,
        outcome_payouts: Vec<Amount>,
    ) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::ProposePayout {
                    market_out_point,
                    outcome_payouts,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::ProposePayout(
                Ok(r),
            ))) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::ProposePayout(
                Err(e),
            ))) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_market_payout_control_proposals(
        &self,
        market: OutPoint,
        from_local_cache: bool,
    ) -> anyhow::Result<BTreeMap<PublicKey, Vec<Amount>>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::GetMarketPayoutControlProposals {
                        market,
                        from_local_cache,
                    },
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetMarketPayoutControlProposals(Ok(r)),
            )) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetMarketPayoutControlProposals(Err(e)),
            )) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_client_payout_control_markets(
        &self,
        from_local_cache: bool,
        markets_created_after_and_including: UnixTimestamp,
    ) -> anyhow::Result<BTreeMap<UnixTimestamp, Vec<OutPoint>>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::GetClientPayoutControlMarkets {
                        from_local_cache,
                        markets_created_after_and_including,
                    },
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetClientPayoutControlMarkets(Ok(r)),
            )) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetClientPayoutControlMarkets(Err(e)),
            )) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn send_payout_control_bitcoin_balance_to_primary_module(
        &self,
    ) -> anyhow::Result<Amount, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::SendPayoutControlBitcoinBalanceToPrimaryModule,
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::SendPayoutControlBitcoinBalanceToPrimaryModule(Ok(r)),
            )) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::SendPayoutControlBitcoinBalanceToPrimaryModule(Err(
                    e,
                )),
            )) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn new_order(
        &self,
        market: OutPoint,
        outcome: Outcome,
        side: Side,
        price: Amount,
        quantity: ContractOfOutcomeAmount,
    ) -> anyhow::Result<OrderIdClientSide, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::NewOrder {
                    market,
                    outcome,
                    side,
                    price,
                    quantity,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::NewOrder(Ok(r)))) => {
                Ok(r)
            }
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::NewOrder(Err(e)))) => {
                Err(RpcError::ClientStopped(e.to_string()))
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_order(
        &self,
        id: OrderIdClientSide,
        from_local_cache: bool,
    ) -> anyhow::Result<Option<Order>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetOrder {
                    id,
                    from_local_cache,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetOrder(Ok(r)))) => {
                Ok(r)
            }
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetOrder(Err(e)))) => {
                Err(RpcError::ClientStopped(e.to_string()))
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn cancel_order(&self, id: OrderIdClientSide) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::CancelOrder { id }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::CancelOrder(Ok(
                r,
            )))) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::CancelOrder(Err(
                e,
            )))) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn send_order_bitcoin_balance_to_primary_module(
        &self,
    ) -> anyhow::Result<Amount, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::SendOrderBitcoinBalanceToPrimaryModule,
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::SendOrderBitcoinBalanceToPrimaryModule(Ok(r)),
            )) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::SendOrderBitcoinBalanceToPrimaryModule(Err(e)),
            )) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn sync_orders(
        &self,
        sync_possible_payouts: bool,
        market: Option<OutPoint>,
        outcome: Option<Outcome>,
    ) -> anyhow::Result<BTreeMap<OrderIdClientSide, Order>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::SyncOrders {
                    sync_possible_payouts,
                    market,
                    outcome,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::SyncOrders(Ok(r)))) => {
                Ok(r)
            }
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::SyncOrders(Err(
                e,
            )))) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_orders_from_db(
        &self,
        market: Option<OutPoint>,
        outcome: Option<Outcome>,
    ) -> anyhow::Result<BTreeMap<OrderIdClientSide, Order>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetOrdersFromDb {
                    market,
                    outcome,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetOrdersFromDb(
                r,
            ))) => Ok(r),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn resync_order_slots(&self, gap_size_to_check: u16) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::ResyncOrderSlots {
                    gap_size_to_check,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::ResyncOrderSlots(
                Ok(r),
            ))) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::ResyncOrderSlots(
                Err(e),
            ))) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_candlesticks(
        &self,
        market: OutPoint,
        outcome: Outcome,
        candlestick_interval: Seconds,
        min_candlestick_timestamp: UnixTimestamp,
    ) -> anyhow::Result<BTreeMap<UnixTimestamp, Candlestick>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetCandlesticks {
                    market,
                    outcome,
                    candlestick_interval,
                    min_candlestick_timestamp,
                }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetCandlesticks(
                Ok(r),
            ))) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetCandlesticks(
                Err(e),
            ))) => Err(RpcError::ClientStopped(e.to_string())),
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
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::WaitCandlesticks(
                Ok(r),
            ))) => Ok(r),
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::WaitCandlesticks(
                Err(e),
            ))) => Err(RpcError::ClientStopped(e.to_string())),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn save_market(&self, market: OutPoint) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::SaveMarket { market }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::SaveMarket)) => Ok(()),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn unsave_market(&self, market: OutPoint) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::UnsaveMarket { market }),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::UnsaveMarket)) => {
                Ok(())
            }
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_saved_markets(
        &self,
    ) -> anyhow::Result<Vec<(OutPoint, UnixTimestamp)>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetSavedMarkets),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(PredictionMarketsRpcResponse::GetSavedMarkets(
                r,
            ))) => Ok(r),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn set_name_to_payout_control(
        &self,
        name: String,
        payout_control: Option<PublicKey>,
    ) -> anyhow::Result<(), RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::SetNameToPayoutControl {
                        name,
                        payout_control,
                    },
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::SetNameToPayoutControl,
            )) => Ok(()),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_name_to_payout_control(
        &self,
        name: String,
    ) -> anyhow::Result<Option<PublicKey>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(
                    PredictionMarketsRpcRequest::GetNameToPayoutControl { name },
                ),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetNameToPayoutControl(r),
            )) => Ok(r),
            _ => Err(RpcError::InvalidResponse),
        }
    }

    pub async fn get_name_to_payout_control_map(
        &self,
    ) -> anyhow::Result<HashMap<String, PublicKey>, RpcError> {
        let (response_sender, response_receiver) = oneshot::channel();
        self.sender
            .send((
                RpcRequest::PredictionMarkets(PredictionMarketsRpcRequest::GetNameToPayoutControlMap),
                response_sender,
            ))
            .await
            .expect("Client has stopped");
        let response = response_receiver.await.expect("Client has stopped");
        match response {
            Ok(RpcResponse::PredictionMarkets(
                PredictionMarketsRpcResponse::GetNameToPayoutControlMap(r),
            )) => Ok(r),
            _ => Err(RpcError::InvalidResponse),
        }
    }
}
