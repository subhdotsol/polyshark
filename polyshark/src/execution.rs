use crate::fees::FeeModel;
use crate::fills::FillModel;
use crate::types::{ExecutionResult, OrderBook, Side};
use crate::wallet::Wallet;

/// Execution simulator
#[derive(Debug)]
pub struct ExecutionEngine {
    pub fee_model: FeeModel,
}

impl ExecutionEngine {
    pub fn new(fee_model: FeeModel) -> Self {
        Self { fee_model }
    }

    /// Simulate order execution
    pub fn execute(
        &self,
        book: &OrderBook,
        size: f64,
        side: Side,
        wallet: &mut Wallet,
    ) -> Option<ExecutionResult> {
        // 1. Check fill ratio
        let filled_size = FillModel::filled_size(book, size, side);
        if filled_size <= 0.0 {
            return None;
        }

        // 2. Calculate execution price (with slippage)
        let exec_price = book.execution_price(filled_size, side)?;
        let midpoint = book.midpoint()?;
        let slippage = ((exec_price - midpoint) / midpoint).abs();

        // 3. Calculate costs
        let notional = exec_price * filled_size;
        let fee = self.fee_model.calculate(notional, false); // Taker
        let total_cost = notional + fee;

        // 4. Check if affordable
        if !wallet.can_afford(total_cost) {
            return None;
        }

        // 5. Execute
        wallet.deduct(total_cost);
        wallet.record_fee(fee);

        Some(ExecutionResult {
            filed_size: filled_size,
            execution_price: exec_price,
            fee_paid: fee,
            slippage,
            total_cost,
            success: true,
        })
    }
}