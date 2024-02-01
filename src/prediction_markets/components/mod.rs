pub mod candlestick_chart;
pub mod home;
pub mod market;
pub mod new_market;
pub mod new_order;
pub mod payout_controls;
pub mod select_market;
pub mod view_market;

pub use candlestick_chart::*;
use fedimint_prediction_markets_common::config::GeneralConsensus;
pub use home::*;
pub use market::*;
pub use new_market::*;
pub use new_order::*;
pub use payout_controls::*;
use secp256k1::PublicKey;
pub use select_market::*;
pub use view_market::*;

#[derive(Debug, Clone)]
pub struct PredictionMarketsStaticDataContext {
    client_payout_control: PublicKey,
    general_consensus: GeneralConsensus,
}
