//! Given a year, report if it is a leap year
// On every year that is evenly divisible by 4
// except every year that is evenly divisible by 100
// unless the year is also evenly divisible by 400
function isLeap(year) {
    var res = false;
    year % 4 == 0 ? year % 100 == 0 ? year % 400 == 0 ? res = true : '' : res = true : '';
    return res;
}
console.log(isLeap(1970), 'false'); // -> false
console.log(isLeap(1996), 'true'); // -> true
console.log(isLeap(1960), 'true'); // -> true
console.log(isLeap(2100), 'false'); // -> false
console.log(isLeap(1900), 'false'); // -> false
console.log(isLeap(2000), 'true'); // -> true
console.log(isLeap(2400), 'true'); // -> true
console.log(isLeap(1800), 'false'); // -> false
