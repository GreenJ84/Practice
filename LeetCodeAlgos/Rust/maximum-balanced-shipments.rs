// You are given an integer array weight of length n, representing the weights of n parcels arranged in a straight line. A shipment is defined as a contiguous subarray of parcels. A shipment is considered balanced if the weight of the last parcel is strictly less than the maximum weight among all parcels in that shipment.

// Select a set of non-overlapping, contiguous, balanced shipments such that each parcel appears in at most one shipment (parcels may remain unshipped).

// Return the maximum possible number of balanced shipments that can be formed.

struct Solution;
impl Solution {
  pub fn max_balanced_shipments(weight: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut max = -1i32;

    for parcel in weight {
      match parcel {
        _x if _x > max => {
          max = parcel;
        },
        _x if _x < max => {
          ans += 1;
          max = -1;
        }
        _ => {}
      }
    }
    ans
  }
}