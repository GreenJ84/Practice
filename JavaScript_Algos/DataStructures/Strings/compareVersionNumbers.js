// Given two strings, version1, and version2, both representing version numbers
// If version1 > version2 return 1; if version1 < version2 return -1; otherwise return 0.

const test1V1 = "0.1";
const test1V2 = "1.1";

const expected1 = -1;

const test2V1 = "1.0.1";
const test2V2 = "1";
const expected2 = 1;

const test3V1 = "7.5.2.4";
const test3V2 = "7.5.3";
const expected3 = -1;

const test4V1 = "7.5.2.4";
const test4V2 = "7.5.2";
const expected4 = 1;

const test5V1 = "1.01";
const test5V2 = "1.001";
const expected5 = 0;
//Ignore leading zeroes, both “01” and “001" represent the same number “1”

const test6V1 = "1.0.1";
const test6V2 = "1";
const expected6 = 1;

//! Determines which version number is greater or if they are equal.
function compareVersionNumbers(v1, v2) {

    let v1arr = v1.split('.').map(item => parseInt(item));
    let v2arr = v2.split('.').map(item => parseInt(item));

    if (v1arr > v2arr){
        return 1
    } else if (v1arr < v2arr){
        return -1
    }
    else {
        return 0
    }
}

console.log(compareVersionNumbers(test1V1, test1V2));
console.log(compareVersionNumbers(test2V1, test2V2));
console.log(compareVersionNumbers(test3V1, test3V2));
console.log(compareVersionNumbers(test4V1, test4V2));
console.log(compareVersionNumbers(test5V1, test5V2));
console.log(compareVersionNumbers(test6V1, test6V2));