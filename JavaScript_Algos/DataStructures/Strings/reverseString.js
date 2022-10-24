// Implement a function reverseString(str) that, given a string, will return the string of the same length but with characters reversed. Example: given ​"creature"​, return ​"erutaerc"​. Do not use the built-in reverse()​ function!

function reverseString(str){
    let i = str.length-1;
    let reverse = "";
    while (i >= 0){
        reverse += str[i];
        i--;
    }
    console.log(reverse);
    return reverse;
}

reverseString('zebra');
reverseString('watermelon');
reverseString('gamecube');
