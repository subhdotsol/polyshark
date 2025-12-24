use serde::{Deserialize, Serialize};


// represents a polymarket prediction market
#[derive(Debug, Clone , Serialize , Deserialize)]
pub struct Market {
    pub id : String , 
    pub question : String , 
    pub slug : String , 
    pub outcomes : Vec<String> , // ["yes" , "no"]
    pub outcome_prices : Vec<f64> , // [0.5 , 0.5]
    pub clob_token_ids : Vec<String> ,  // Token Ids for trading 
    pub best_bid : Option<f64> , 
    pub best_ask : Option<f64> , 
    pub maker_base_fee : u32 ,   // In basis points (eg : 0)
    pub taker_base_fee : u32 , // In basis points (eg : 200 = 2%)
    pub liquidity : f64 , 
    pub volume_24hr : f64 , 
    pub active : bool , 
    pub accepting_orders : bool 
}

// Single price level in order book 
#[derive(Debug, Clone , Serialize , Deserialize)]
pyb struct PriceLevel { 
    pub price : f64 , 
    pub size : u64 
}

// Order book for a single token 
#[derive(Debug, Clone , Serialize , Deserialize)]
pub struct OrderBook { 
    pub token_id : String , 
    pub bids : Vec<PriceLevel> , 
    pub asks : Vec<PriceLevel> 
    pub timestamp : u64 
}

// Executed trade records 
#[derive(Debug, Clone , Serialize , Deserialize)]
pub struct Trade {
    pub id : String , 
    pub token_id : String , 
    pub price : f64  
    pub size : f64 , 
    pub side : Side , 
    pub timestamp : u64 
}

// Order side 
#[derive(Debug, Clone, PartialEq , Serialize , Deserialize)]
pub enum Side { 
    Buy , 
    Sell 
}

// Arbitrage signal
#[derive(Debug, Clone)]
pub struct ArbitrageSignal {
    pub market_id : String , 
    pub spread : f64 ,  // how much the price deviates from 1 
    pub edge : f64 , // Expected profit per unit 
    pub recommended_side : Side , 
    pub yes_price : f64 , 
    pub no_price : f64 
}

// Execution resutl 
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub filed_size : f64  , 
    pub execution_price : f64 , 
    pub fee_paid : f64 , 
    pub slippage : f64 , 
    pub total_cost : f64 , 
    pub success : bool
}


