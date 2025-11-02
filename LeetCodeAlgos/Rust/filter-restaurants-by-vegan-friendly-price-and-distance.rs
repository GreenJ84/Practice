// Given the array restaurants where  restaurants[i] = [idi, ratingi, veganFriendlyi, pricei, distancei]. You have to filter the restaurants using three filters.

// The veganFriendly filter will be either true (meaning you should only include restaurants with veganFriendlyi set to true) or false (meaning you can include any restaurant). In addition, you have the filters maxPrice and maxDistance which are the maximum value for price and distance of restaurants you should consider respectively.

// Return the array of restaurant IDs after filtering, ordered by rating from highest to lowest. For restaurants with the same rating, order them by id from highest to lowest. For simplicity veganFriendlyi and veganFriendly take value 1 when it is true, and 0 when it is false.

struct Solution;
impl Solution {
    pub fn filter_restaurants(restaurants: Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
      let mut filtered = restaurants.into_iter()
        .filter(|rest|{
          rest[2] >= vegan_friendly &&
          rest[3] <= max_price &&
          rest[4] <= max_distance
        })
        .collect::<Vec<Vec<i32>>>();

      filtered.sort_by(|aRest, bRest| {
        aRest[1].cmp(&bRest[1])
          .then(aRest[0].cmp(&bRest[0]))
          .reverse()
      });

      filtered.into_iter()
        .map(|rest| rest[0])
        .collect::<Vec<i32>>()
    }
}