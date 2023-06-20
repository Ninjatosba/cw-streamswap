use crate::state::Status;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Decimal256, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    /// Minimum sale duration in blocks
    pub min_stream_blocks: u64,
    /// Minimum duration between start block and current block in unix seconds
    pub min_blocks_until_start_block: u64,
    /// Accepted stream creation fee denom
    pub stream_creation_denom: String,
    /// Stream creation fee amount
    pub stream_creation_fee: Uint128,
    /// in/buy token exit fee in percent
    pub exit_fee_percent: Decimal,
    /// Address of the fee collector
    pub fee_collector: String,
    /// protocol admin can pause streams in case of emergency.
    pub protocol_admin: String,
    /// Accepted in_denom to buy out_tokens
    pub accepted_in_denom: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// CreateStream creates new token stream. Anyone can create a new stream.
    /// Creation Fee send along msg prevents spams.
    CreateStream {
        /// Address where the stream earnings will be sent.
        treasury: String,
        /// Name of the stream.
        name: String,
        /// An external resource describing a stream.
        url: Option<String>,
        /// Payment denom - used to buy `token_out`.
        /// Also known as quote currency.
        in_denom: String,
        /// Denom to stream (distributed to the investors).
        /// Also known as a base currency.
        out_denom: String,
        /// Total number of `token_out` to be sold during the continuous stream.
        out_supply: Uint128,
        /// Unix timestamp when the stream starts. Calculations in nano sec precision.
        start_block: u64,
        /// Unix timestamp when the stream ends. Calculations in nano sec precision.
        end_block: u64,
    },
    /// Update stream and calculates distribution state.
    UpdateStream {
        stream_id: u64,
    },
    // Update protocol admin, only authorized admin can update.
    UpdateProtocolAdmin {
        new_protocol_admin: String,
    },
    /// UpdateOperator updates the operator of the position.
    UpdateOperator {
        stream_id: u64,
        new_operator: Option<String>,
    },
    /// Subscribe to a token stream. Any use at any time before the stream end can join
    /// the stream by sending `token_in` to the Stream through the Subscribe msg.
    /// During the stream, user `token_in` will be automatically charged every
    /// epoch to purchase `token_out`.
    Subscribe {
        stream_id: u64,
        /// operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
        /// operator can subscribe/withdraw/update position.
        operator: Option<String>,
    },
    /// Withdraw unspent tokens in balance.
    Withdraw {
        stream_id: u64,
        cap: Option<Uint128>,
        /// operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
    },
    /// UpdatePosition updates the position of the user.
    /// syncs position index to the current state of the stream.
    UpdatePosition {
        stream_id: u64,
        /// operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
    },
    /// FinalizeStream clean ups the stream and sends income (earned tokens_in) to the
    /// Stream recipient. Returns error if called before the Stream end. Anyone can
    /// call this method.
    FinalizeStream {
        stream_id: u64,
        new_treasury: Option<String>,
    },
    /// ExitStream withdraws (by a user who subscribed to the stream) purchased
    /// tokens_out from the pool and remained tokens_in. Must be called after
    /// the stream ends.
    ExitStream {
        stream_id: u64,
        /// operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
    },
    //
    // Killswitch features
    //
    /// PauseStream pauses the stream. Only protocol admin and governance can pause the stream.
    PauseStream {
        stream_id: u64,
    },
    /// WithdrawPaused is used to withdraw unspent position funds during pause.
    WithdrawPaused {
        stream_id: u64,
        cap: Option<Uint128>,
        // operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
    },
    /// ExitCancelled returns the whole balance user put in the stream, both spent and unspent.
    ExitCancelled {
        stream_id: u64,
        /// operator_target is the address of operator targets to execute on behalf of the user.
        operator_target: Option<String>,
    },

    UpdateConfig {
        min_stream_blocks: Option<u64>,
        min_blocks_until_start: Option<u64>,
        stream_creation_denom: Option<String>,
        stream_creation_fee: Option<Uint128>,
        fee_collector: Option<String>,
        accepted_in_denom: Option<String>,
        exit_fee_percent: Option<Decimal>,
    },
    ResumeStream {
        stream_id: u64,
    },
    CancelStream {
        stream_id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns current configuration.
    #[returns(ConfigResponse)]
    Config {},
    /// Returns a stream's current state.
    #[returns(StreamResponse)]
    Stream { stream_id: u64 },
    /// Returns list of streams paginated by `start_after` and `limit`.
    #[returns(StreamsResponse)]
    ListStreams {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    /// Returns current state of a position.
    #[returns(PositionResponse)]
    Position { stream_id: u64, owner: String },
    /// Returns list of positions paginated by `start_after` and `limit`.
    #[returns(PositionsResponse)]
    ListPositions {
        stream_id: u64,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns average price of a stream sale.
    #[returns(AveragePriceResponse)]
    AveragePrice { stream_id: u64 },
    /// Returns currently streaming price of a sale.
    #[returns(LatestStreamedPriceResponse)]
    LastStreamedPrice { stream_id: u64 },
}

#[cw_serde]
pub struct ConfigResponse {
    /// Minimum time in seconds for a stream to last.
    pub min_stream_blocks: u64,
    /// Minimum time in seconds until the start time of a stream.
    pub min_blocks_until_start: u64,
    /// Denom accepted for subscription.
    pub accepted_in_denom: String,
    /// Denom used as fee for creating a stream.
    pub stream_creation_denom: String,
    /// Creation fee amount.
    pub stream_creation_fee: Uint128,
    /// This percentage represents the fee that will be collected from the investors.
    pub exit_fee_percent: Decimal,
    /// Address of the fee collector.
    pub fee_collector: String,
    /// Address of the protocol admin.
    pub protocol_admin: String,
}

#[cw_serde]
pub struct StreamResponse {
    pub id: u64,
    /// address of the treasury where the stream earnings will be sent.
    pub treasury: String,
    /// URL of the stream.
    pub url: Option<String>,
    /// Proportional distribution variable to calculate the distribution of in token_out to buyers.
    pub dist_index: Decimal256,
    /// last updated time of stream.
    pub last_updated_block: u64,
    /// denom of the `token_out`.
    pub out_denom: String,
    /// total number of `token_out` to be sold during the continuous stream.
    pub out_supply: Uint128,
    /// total number of remaining out tokens at the time of update.
    pub out_remaining: Uint128,
    /// denom of the `token_in`.
    pub in_denom: String,
    /// total number of `token_in` on the buy side at latest state.
    pub in_supply: Uint128,
    /// total number of `token_in` spent at latest state.
    pub spent_in: Uint128,
    /// total number of shares minted.
    pub shares: Uint128,
    /// start time when the token emission starts. in nanos.
    pub start_block: u64,
    /// end time when the token emission ends.
    pub end_block: u64,
    /// price at when latest distribution is triggered.
    pub current_streamed_price: Decimal,
    /// Status of the stream. Can be `Waiting`, `Active`, `Finalzed`, `Paused` or `Canceled` for kill switch.
    pub status: Status,
    /// Date when the stream was paused.
    pub pause_block: Option<u64>,
    /// Exit fee percent.
    pub exit_fee_percent: Decimal,
    /// Creation fee amount.
    pub stream_creation_fee: Uint128,
}

#[cw_serde]
pub struct StreamsResponse {
    pub streams: Vec<StreamResponse>,
}

#[cw_serde]
pub struct PositionResponse {
    pub stream_id: u64,
    /// creator of the position.
    pub owner: String,
    /// current amount of tokens in buy pool
    pub in_balance: Uint128,
    pub shares: Uint128,
    // index is used to calculate the distribution a position has
    pub index: Decimal256,
    pub last_updated_block: u64,
    // total amount of `token_out` purchased in tokens at latest calculation
    pub purchased: Uint128,
    // pending purchased accumulates purchases after decimal truncation
    pub pending_purchase: Decimal256,
    // total amount of `token_in` spent tokens at latest calculation
    pub spent: Uint128,
    // operator can update position
    pub operator: Option<Addr>,
}

#[cw_serde]
pub struct PositionsResponse {
    pub positions: Vec<PositionResponse>,
}

#[cw_serde]
pub struct AveragePriceResponse {
    pub average_price: Decimal,
}

#[cw_serde]
pub struct LatestStreamedPriceResponse {
    pub current_streamed_price: Decimal,
}

#[cw_serde]
pub enum SudoMsg {
    PauseStream { stream_id: u64 },
    CancelStream { stream_id: u64 },
    ResumeStream { stream_id: u64 },
}

#[cw_serde]
pub struct MigrateMsg {}
