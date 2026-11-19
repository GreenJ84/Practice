// There are n cities connected by some number of flights. You are given an array flights where flights[i] = [fromi, toi, pricei] indicates that there is a flight from city fromi to city toi with cost pricei.

// You are also given three integers src, dst, and k, return the cheapest price from src to dst with at most k stops. If there is no such route, return -1.

// Constraints:
// 2 <= n <= 100
// 0 <= flights.length <= (n * (n - 1) / 2)
// flights[i].length == 3
// 0 <= fromi, toi < n
// fromi != toi
// 1 <= pricei <= 104
// There will not be any multiple flights between two cities.
// 0 <= src, dst, k < n
// src != dst

struct Solution;

struct Route(pub i32, pub i32);
struct Flight(pub i32, pub i32);
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let dst = dst as usize;

        let mut prices = vec![i32::MAX; n as usize];
        prices[src as usize] = 0;
        // At most k stops means at most k + 1 flight segments
        for _ in 0..=k as usize {
            let mut next_prices = prices.clone();
            let mut updated = false;

            for flight in &flights {
                let from = flight[0] as usize;
                let to = flight[1] as usize;
                let cost = flight[2];

                if prices[from] != i32::MAX && prices[from].saturating_add(cost) < next_prices[to] {
                    next_prices[to] = prices[from] + cost;
                    updated = true;
                }
            }

            prices = next_prices;
            if !updated {
                break;
            }
        }

        if prices[dst] == i32::MAX {
            -1
        } else {
            prices[dst]
        }
    }

    // TIME LIMIT EXCEEDED
    // This solution uses a recursive approach to explore all possible paths from the source to the destination with exponential path blowup. It does not use any memoization or dynamic programming techniques to optimize the search, leading to inefficiency for larger inputs.
    pub fn find_cheapest_price1(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut depart = HashMap::<i32, Vec<Flight>>::with_capacity(n as usize);
        flights.into_iter().for_each(|flight| {
            let departures = depart
                .entry(flight[0])
                .or_insert_with(|| vec![]);
            departures.push(Flight(flight[1], flight[2]));
        });

        let mut cost = 0;
        let mut seen = HashSet::<i32>::new();
        fn travel(
            source: i32,
            curr: Route,
            k: i32,
            dst: i32,
            depart: &HashMap<i32, Vec<Flight>>,
            seen: &mut HashSet<i32>,
            cost: &mut i32,
        ) {
            if curr.0 > k || (cost != &0 && curr.1 >= *cost) {
                return;
            }
            seen.insert(source);
            if let Some(destinations) = depart.get(&source) {
                for dest in destinations {
                    if dest.0 == dst {
                        if cost == &0 {
                            *cost = curr.1 + dest.1;
                        } else {
                            *cost = (*cost).min(curr.1 + dest.1);
                        }
                        continue;
                    } else if seen.contains(&dest.0) {
                        continue;
                    }
                    travel(dest.0, Route(curr.0 + 1, curr.1 + dest.1), k, dst, depart, seen, cost);
                }
            }
            seen.remove(&source);
        }
        travel(src, Route(0, 0), k, dst, &depart, &mut seen, &mut cost);

        if cost == 0 {
            return -1;
        }
        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let n = 4;
        let flights = vec![
            vec![0, 1, 100],
            vec![1, 2, 100],
            vec![2, 0, 100],
            vec![1, 3, 600],
            vec![2, 3, 200],
        ];
        let src = 0;
        let dst = 3;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 700);
    }

    #[test]
    fn test_2() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 1;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 200);
    }

    #[test]
    fn test_3() {
        let n = 3;
        let flights = vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]];
        let src = 0;
        let dst = 2;
        let k = 0;
        assert_eq!(Solution::find_cheapest_price(n, flights, src, dst, k), 500);
    }
}
