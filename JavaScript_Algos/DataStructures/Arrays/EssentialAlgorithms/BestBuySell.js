// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

var maxProfit = function(prices) {
    let max = 0;
    let smallestDay = prices[0];
    let i = 1;

    while( i < prices.length ){
        if (prices[i] > smallestDay){
            if (prices[i] - smallestDay > max){
                max = prices[i] - smallestDay;
            }
        } else if(prices[i] < smallestDay){
            smallestDay = prices[i];
        }
        i++;
    }
    return max;
};
