/// Stockmax solver
///
/// ```
/// assert_eq!(stockmax::stockmax(vec![5, 3, 2]), 0);
/// ```
///
/// ```
/// assert_eq!(stockmax::stockmax(vec![1, 2, 100]), 197);
/// ```
///
/// ```
/// assert_eq!(stockmax::stockmax(vec![1, 3, 1, 2]), 3);
/// ```
pub fn stockmax (prices: Vec<u32>) -> u64 {
  stockmax_inner(&prices)
}

fn stockmax_inner (prices: &[u32]) -> u64 {
  if prices.len() == 0 {
    return 0;
  }
  let mut price_max = prices[0];
  let mut idx_price_max = 0;
  for (idx_price_cur, price_cur) in prices.iter().enumerate().skip(1) {
    if *price_cur > price_max {
      price_max = *price_cur;
      idx_price_max = idx_price_cur;
    }
  }
  let spend: u32 = prices.iter().take(idx_price_max).sum();
  let profit: u64 = (u64::from(price_max) * (idx_price_max as u64)) - u64::from(spend);
  return profit + stockmax_inner(&prices[(idx_price_max + 1)..]);
}
