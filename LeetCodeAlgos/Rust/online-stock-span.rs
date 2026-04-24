// Design an algorithm that collects daily price quotes for some stock and returns the span of that stock's price for the current day.

// The span of the stock's price in one day is the maximum number of consecutive days (starting from that day and going backward) for which the stock price was less than or equal to the price of that day.

// For example, if the prices of the stock in the last four days is [7,2,1,2] and the price of the stock today is 2, then the span of today is 4 because starting from today, the price of the stock was less than or equal 2 for 4 consecutive days.
// Also, if the prices of the stock in the last four days is [7,34,1,2] and the price of the stock today is 8, then the span of today is 3 because starting from today, the price of the stock was less than or equal 8 for 3 consecutive days.
// Implement the StockSpanner class:

// StockSpanner() Initializes the object of the class.
// int next(int price) Returns the span of the stock's price given that today's price is price.

struct StockSpanner {
  last_prices: Vec<(i32, i32)>,
}
impl StockSpanner {
    fn new() -> Self {
        Self{ last_prices: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        let mut span = 1;
        while let Some((last_price, last_span)) = self.last_prices.last() {
          if *last_price <= price {
            span += last_span;
            self.last_prices.pop();
          } else {
            break;
          }
        }
        self.last_prices.push((price, span));
        span
    }
}

// struct StockSpanner {
//   current_span: i32,
//   last_prices: Vec<i32>,
// }
// impl StockSpanner {
//     fn new() -> Self {
//         Self{ current_span: 0, last_prices: vec![] }
//     }

//     fn next(&mut self, price: i32) -> i32 {
//         if self.last_prices.len() == 0 || price < self.last_prices[self.last_prices.len() - 1] {
//           self.current_span = 1;
//         } else {
//           self.current_span = {
//             let mut span = 1;
//             for i in (0..self.last_prices.len()).rev() {
//               if self.last_prices[i] <= price {
//                 span += 1;
//               } else {
//                 break;
//               }
//             }
//             span
//           };
//         }
//         self.last_prices.push(price);
//         self.current_span
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(100), 1);
        assert_eq!(stock_spanner.next(80), 1);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(70), 2);
        assert_eq!(stock_spanner.next(60), 1);
        assert_eq!(stock_spanner.next(75), 4);
        assert_eq!(stock_spanner.next(85), 6);
    }

    #[test]
    fn test_2() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(10), 1);
        assert_eq!(stock_spanner.next(20), 2);
        assert_eq!(stock_spanner.next(30), 3);
        assert_eq!(stock_spanner.next(25), 1);
        assert_eq!(stock_spanner.next(35), 5);
    }

    #[test]
    fn test_3() {
        let mut stock_spanner = StockSpanner::new();
        assert_eq!(stock_spanner.next(5), 1);
        assert_eq!(stock_spanner.next(4), 1);
        assert_eq!(stock_spanner.next(3), 1);
        assert_eq!(stock_spanner.next(2), 1);
        assert_eq!(stock_spanner.next(1), 1);
    }
}


// Example 1:

// Input
// ["StockSpanner", "next", "next", "next", "next", "next", "next", "next"]
// [[], [100], [80], [60], [70], [60], [75], [85]]
// Output
// [null, 1, 1, 1, 2, 1, 4, 6]

// Explanation
// StockSpanner stockSpanner = new StockSpanner();
// stockSpanner.next(100); // return 1
// stockSpanner.next(80);  // return 1
// stockSpanner.next(60);  // return 1
// stockSpanner.next(70);  // return 2
// stockSpanner.next(60);  // return 1
// stockSpanner.next(75);  // return 4, because the last 4 prices (including today's price of 75) were less than or equal to today's price.
// stockSpanner.next(85);  // return 6