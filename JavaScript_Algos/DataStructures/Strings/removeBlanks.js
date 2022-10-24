// Create a function that, given a string, returns the string, without blanks. Given ​" play that Funky Music "​, returns a string containing ​"playthatFunkyMusic"​.

function removeBlanks(str){
    let i = 0;
    let noBlanks = ''
    while (i < str.length){
        if (str[i] != ' '){
            noBlanks += str[i];
        }
        i++;
    }
    console.log(noBlanks);
    return noBlanks;
}

removeBlanks(" play that Funky Music ");