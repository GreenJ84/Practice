// When given an array of words, returns the largest suffix (word-end) that is common between all words. For example, for input ​["ovation", "notation", "action"]​, return ​"tion"​. Given ​["nice", "ice", "sic"]​, return ​""​.

function commonSuffix(arr){
    let result = [''];
    if (arr.length < 1){ return result.join() }

    result = arr[0].split('').reverse();
    for (let i = 1; i < arr.length; i++){
        let j = 0;
        let arrJ = arr[i].split('').reverse();

        while(j < arr[i].length && arrJ[j] === result[j] ){
            j++;
        }
        if (j < result.length){ result.splice(j, result.length-j); }
        if ( result.length < 1 ){ return result }
    }
    result = result.reverse().join('');
    
    return result;
}

console.log(commonSuffix(["ovation", "notation", "action"]));
console.log(commonSuffix(["nice", "ice", "sic"]));