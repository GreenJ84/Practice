// Strings like ​"Able was I, ere I saw Elba"​ or ​"Madam, I'm Adam"​ could be considered ​palindromes​, because (if we ignore spaces, punctuation and capitalization) the letters are the same from front and back.
// Create a function that returns a boolean whether the string is a ​strict​ palindrome. For ​"a x a"​ or "racecar"​, return ​true​. Do ​not​ ignore spaces, punctuation and capitalization: if given ​"Dud"​ or "oho!"​, return ​false​. Example abve will return false.
// Create a function that returns a boolean whether the string is a palindrome. For ​"a x a"​ or "racecar" and the ones above​, return ​true​. Ignore spaces, punctuation and capitalization: if given ​"Dud"​ or "oho!"​, return ​false​.

function strictPal(str){
    let i = str.length-1;
    let valid = true;
    str.split('').forEach((item) => {
        if( str[i] != item ){
            valid = false;
        }
        i--;
    })
    return valid;
}

console.log(strictPal("Dud"));
console.log(strictPal("oho!"));
console.log(strictPal("a x a"));
console.log(strictPal("Madam, I'm Adam"));
console.log(strictPal("Able was I, ere I saw Elba"));