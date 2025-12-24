# 📐 PolyShark — Math Formulas

All the key formulas used in the PolyShark arbitrage bot.

---

## 1. Best Bid & Best Ask

```
Best Bid = MAX(all bid prices)     // Highest price someone will BUY at
Best Ask = MIN(all ask prices)     // Lowest price someone will SELL at
```

In a sorted order book:
```rust
best_bid = bids[0].price    // First bid (highest)
best_ask = asks[0].price    // First ask (lowest)
```

**Example:**
| Bids | Asks |
|------|------|
| 0.49 | 0.51 |
| 0.48 | 0.52 |
| 0.47 | 0.53 |

→ Best Bid = **0.49**, Best Ask = **0.51**

---

## 2. Midpoint Price

```
Midpoint = (Best_Bid + Best_Ask) / 2
```

**Example:**
```
Midpoint = (0.49 + 0.51) / 2 = 0.50
```

---

## 3. Bid-Ask Spread

```
Spread = Best_Ask - Best_Bid
```

**Example:**
```
Spread = 0.51 - 0.49 = 0.02 (2 cents)
```

---

## 4. Arbitrage Detection (Core Formula)

### The Invariant
```
YES_price + NO_price = 1.0    (No arbitrage)
YES_price + NO_price ≠ 1.0    (Arbitrage exists!)
```

### Arbitrage Spread
```
Spread = |1.0 - (YES_price + NO_price)|
```

**Example:**
```
YES = 0.48
NO  = 0.47
Sum = 0.95

Spread = |1.0 - 0.95| = 0.05 (5% profit opportunity!)
```

### Why This Works
- One outcome MUST happen (YES or NO)
- Winner pays out $1.00
- If you buy both for $0.95, you profit $0.05 guaranteed

---

## 5. Fee Calculation

### Convert Basis Points to Decimal
```
Fee_Rate = taker_base_fee / 10000
```

**Example:**
```
200 basis points = 200 / 10000 = 0.02 = 2%
```

### Calculate Fee
```
Fee = Notional × Fee_Rate
```

**Example:**
```
Notional = $100
Fee_Rate = 0.02
Fee = 100 × 0.02 = $2.00
```

---

## 6. Total Liquidity

```
Bid_Liquidity = Σ (size of all bids)
Ask_Liquidity = Σ (size of all asks)
```

**Example:**
```
Bids: [500, 800, 300] → Total = 1600 tokens
Asks: [400, 700, 200] → Total = 1300 tokens
```

---

## 7. Execution Price (VWAP)

Volume-Weighted Average Price — what you actually pay when buying.

### Algorithm
```
For buying SIZE tokens:
    remaining = SIZE
    total_cost = 0
    
    For each ask level (lowest price first):
        fill = min(remaining, level.size)
        total_cost += fill × level.price
        remaining -= fill
        if remaining == 0: break
    
    VWAP = total_cost / SIZE
```

**Example:** Buy 600 tokens
```
Asks:
  0.51 × 400 tokens = $204
  0.52 × 200 tokens = $104 (only need 200 more)
  
Total Cost = $308
VWAP = 308 / 600 = $0.5133
```

---

## 8. Slippage

How much worse your execution price is vs the midpoint.

```
Slippage = |Execution_Price - Midpoint| / Midpoint
```

**Example:**
```
Midpoint = 0.50
Execution = 0.52

Slippage = |0.52 - 0.50| / 0.50 = 0.04 = 4%
```

---

## 9. Expected Profit (Decision Gate)

### Full Formula
```
Gross_Edge = Arbitrage_Spread × Size

Fee_Cost = Size × Avg_Price × Fee_Rate × 2   // ×2 for both YES and NO

Slippage_Cost = Size × Slippage

Expected_Profit = Gross_Edge - Fee_Cost - Slippage_Cost
```

### Decision Rule
```
IF Expected_Profit > 0  → EXECUTE TRADE
IF Expected_Profit ≤ 0  → SKIP TRADE
```

**Example:**
```
Spread = 0.05 (5%)
Size = 100 tokens
Avg_Price = 0.475
Fee_Rate = 0.02
Slippage = 0.01

Gross_Edge = 0.05 × 100 = $5.00
Fee_Cost = 100 × 0.475 × 0.02 × 2 = $1.90
Slippage_Cost = 100 × 0.01 = $1.00

Expected_Profit = 5.00 - 1.90 - 1.00 = $2.10 ✅ TRADE!
```

---

## 10. Total Trade Cost

```
Total_Cost = (Execution_Price × Size) + Fee
```

**Example:**
```
Execution_Price = 0.52
Size = 100
Fee = $2.00

Total_Cost = (0.52 × 100) + 2.00 = $54.00
```

---

## 11. Fill Ratio

How much of your order can actually fill.

```
Fill_Ratio = min(1.0, Available_Liquidity / Requested_Size)
Filled_Size = Requested_Size × Fill_Ratio
```

**Example:**
```
Want to buy: 1000 tokens
Available asks: 600 tokens

Fill_Ratio = 600 / 1000 = 0.6 (60%)
Filled_Size = 1000 × 0.6 = 600 tokens
```

---

## 12. Profit/Loss (PnL)

### For a Long (Buy) Position
```
PnL = (Exit_Price - Entry_Price) × Size
```

### For a Short (Sell) Position
```
PnL = (Entry_Price - Exit_Price) × Size
```

---

## 13. Win Rate

```
Win_Rate = Winning_Trades / Total_Trades
```

**Target:** Win Rate > 50% after all costs

---

## 14. Equity

```
Equity = Cash + Σ(Position_Size × Current_Price)
```

---

## Quick Reference Card

| Formula | Equation |
|---------|----------|
| Midpoint | `(bid + ask) / 2` |
| Spread | `ask - bid` |
| Arb Spread | `|1 - (YES + NO)|` |
| Fee | `notional × (bps / 10000)` |
| VWAP | `Σ(fill × price) / total_size` |
| Slippage | `|exec - mid| / mid` |
| Expected Profit | `edge - fees - slippage` |
| Fill Ratio | `liquidity / size` |
