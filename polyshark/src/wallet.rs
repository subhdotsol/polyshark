use std::collections::HashMap;
use crate::types::Side;


#[derive(Debug, Clone)]
// fake wallet just a variable
pub struct Wallet {
    pub usdc: f64,                              // cash balance
    pub positions: HashMap<String, Position>,   // token_id -> Position
    pub starting_balance: f64,
    pub total_fees_paid: f64,
    pub total_trades: u32,
    pub winning_trades: u32,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub token_id: String,
    pub side: Side,          // Changed from String to Side
    pub size: f64,
    pub entry_price: f64,
    pub entry_time: u64,
}

impl Wallet {
    /// Create new wallet with starting balance
    pub fn new(starting_balance: f64) -> Self {
        Self {
            usdc: starting_balance,
            positions: HashMap::new(),
            starting_balance,
            total_fees_paid: 0.0,
            total_trades: 0,
            winning_trades: 0,
        }
    }

    /// Check if wallet can afford a purchase
    pub fn can_afford(&self, amount: f64) -> bool {
        self.usdc >= amount
    }

    /// Deduct amount from wallet
    pub fn deduct(&mut self, amount: f64) -> bool {
        if self.can_afford(amount) {
            self.usdc -= amount;
            true
        } else {
            false
        }
    }

    /// Credit amount to wallet
    pub fn credit(&mut self, amount: f64) {
        self.usdc += amount;
    }

    /// Add fee to tracking
    pub fn record_fee(&mut self, fee: f64) {
        self.total_fees_paid += fee;
    }

    /// Record a trade result
    pub fn record_trade(&mut self, is_winner: bool) {
        self.total_trades += 1;
        if is_winner {
            self.winning_trades += 1;
        }
    }

    /// Get current equity (cash + position value)
    pub fn equity(&self, current_prices: &HashMap<String, f64>) -> f64 {
        let position_value: f64 = self.positions.iter()
            .map(|(token_id, pos)| {
                let current_price = current_prices.get(token_id).unwrap_or(&0.0);
                pos.size * current_price
            })
            .sum();
        self.usdc + position_value
    }

    /// Get profit/loss from starting balance
    pub fn pnl(&self, current_prices: &HashMap<String, f64>) -> f64 {
        self.equity(current_prices) - self.starting_balance
    }

    /// Get win rate
    pub fn win_rate(&self) -> f64 {
        if self.total_trades == 0 {
            0.0
        } else {
            self.winning_trades as f64 / self.total_trades as f64
        }
    }

    /// Open a new position
    pub fn open_position(&mut self, token_id: String, side: Side, size: f64, price: f64, timestamp: u64) {
        self.positions.insert(token_id.clone(), Position {
            token_id,
            side,
            size,
            entry_price: price,
            entry_time: timestamp,
        });
    }

    /// Close a position and return PnL
    pub fn close_position(&mut self, token_id: &str, exit_price: f64) -> Option<f64> {
        if let Some(pos) = self.positions.remove(token_id) {
            let pnl = match pos.side {
                Side::Buy => (exit_price - pos.entry_price) * pos.size,
                Side::Sell => (pos.entry_price - exit_price) * pos.size,
            };
            self.credit(pos.size * exit_price);
            Some(pnl)
        } else {
            None
        }
    }
}