// Create a JavaScript function that given a string, returns the integer made from the string’s digits. Given "0s1a3y5w7h9a2t4?6!8?0"​, the function should return the number 1,357,924,680.

function getDigits(str){
    let i = 0;
    let numbers = []
    while (i < str.length){
        if (parseInt(str[i]) >= 0){
            numbers.push(parseInt(str[i]))
        }
        i++;
    }
    i = 0;
    while (numbers[i] === 0){
        numbers.splice(i, 1);
    }
    i = numbers.length-1;
    let comma = 0;
    let result = '';
    while ( i >= 0 ){
        if (comma > 0 && comma % 3 === 0){
            result = ','+ result;
        }
        result = numbers[i] + result;
        i--;
        comma++;
    }
    console.log(result);
    return result;
}

getDigits("0s1a3y5w7h9a2t4?6!8?0");
getDigits("0000abc52s14a3y5w7ha2t3?6!8?1");