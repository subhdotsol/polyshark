/// Binary market constraint checker
#[derive(Debug, Clone)]
pub struct ConstraintChecker {
    pub min_spread_threshold: f64,  // e.g., 0.02 for 2%
}

impl ConstraintChecker {
    pub fn new(min_spread_threshold: f64) -> Self {
        Self { min_spread_threshold }
    }

    /// Check if market has arbitrage opportunity
    pub fn check_violation(&self, market: &Market) -> Option<ArbitrageSignal> {
        let spread = market.get_spread();
        
        if spread <= self.min_spread_threshold {
            return None; // No opportunity
        }

        let sum = market.yes_price() + market.no_price();
        let recommended_side = if sum > 1.0 {
            Side::Sell // Prices are overvalued
        } else {
            Side::Buy  // Prices are undervalued
        };

        Some(ArbitrageSignal {
            market_id: market.id.clone(),
            spread,
            edge: spread, // Gross edge before costs
            recommended_side,
            yes_price: market.yes_price(),
            no_price: market.no_price(),
        })
    }
}