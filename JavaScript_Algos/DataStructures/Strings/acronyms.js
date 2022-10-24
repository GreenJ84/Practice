// Create a function that, given a string, returns the string’s acronym (first letters only, capitalized). Given "there's no free lunch - gotta pay yer way"​, return ​"TNFL-GPYW"​. Given ​"Live from New York, it's Saturday Night!"​, return ​"LFNYISN"​.

function acronyms(str){
    let result = '';
    if (str[0] != " "){
        result += str[0];
    }
    let i = 0
    while ( i < str.length ){
        if (str[i] === ' '){
            result += str[i+1];
        }
        i++;
    }
    result = result.toUpperCase();
    console.log(result);
    return result;
}

acronyms("there's no free lunch - gotta pay yer way");
acronyms("Live from New York, it's Saturday Night!")