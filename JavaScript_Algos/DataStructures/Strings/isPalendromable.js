// Given a string return whether or not it's possible to make a palindrome out of the string's characters.
// What is it about a string that makes it possible for it to become a palindrome?

const str1 = "";
const expected1 = false;

const str2 = "a";
const expected2 = true;

const str3 = "ddaa";
const expected3 = true;
// Explanation: "daad" or "adda"

const str4 = "dda";
const expected4 = true;
// Explanation: "dad"

const str5 = "aaadd";
const expected5 = true;
// Explanation: "daaad"

const str6 = "abc";
const expected6 = false;


// Determines whether or not it is possible for the string's characters to be rearranged into a palindrome.
const canStringBecomePalindrome = (str) => {
    let counter = {};
    if (str.length == 0){ return false }
    for(let i = 0; i < str.length; i++){
        if (counter[str[i]]){
            counter[str[i]] += 1;
        } else{
            counter[str[i]] = 1;
        }
    }
    let x = 2;
    Object.keys(counter).forEach(key => {
        counter[key] % 2 == 1 ? x-- : ''
    });

    return x <= 0 ?  false : true;
}
console.log(canStringBecomePalindrome(str1));
console.log(canStringBecomePalindrome(str2));
console.log(canStringBecomePalindrome(str3));
console.log(canStringBecomePalindrome(str4));
console.log(canStringBecomePalindrome(str5));
console.log(canStringBecomePalindrome(str6));

