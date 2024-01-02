// Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.
// Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content with; and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.

/**
 * @param {number[]} g
 * @param {number[]} s
 * @return {number}
 */
const findContentChildren = function(g, s) {
  g.sort((a, b) => {
      return a - b;
  });
  s.sort((a, b) => {
      return a - b;
  });
  let content = 0;
  for (let child = 0; child < g.length; child++){
      //Start from smallest greed
      for (let cookie = 0; cookie < s.length; cookie++){
          if (s[cookie] >= g[child]){
              content++;
              s.splice(cookie, 1);
              break;
          }
      } 
  }
  return content;
};

console.log(
  `Test 1 ${findContentChildren([1, 2, 3], [1, 1]) === 1 ? "passed" : "failed"}`
);
console.log(
  `Test 2 ${findContentChildren([1, 2], [1, 2, 3]) === 2 ? "passed" : "failed"}`
);