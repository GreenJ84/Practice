const str1 = "b70a164c32a20c10";
const expected1 = "a184b70c42";

// Rehashes an incorrectly hashed string by combining letter count occurrences and alphabetizing them.
function rehash(s) {
    let re = /[a-zA-z]/
    let idx = 0;
    let split = idx;
    let result = {};
    for (let char of s){
        if( char.match(re) && result[s[split]] == undefined && char !== s[0]){
            let x = Number(s.slice(split+1, idx));
            result[s[split]] = Number(s.slice(split+1, idx));
            split = idx;
        } else if (result.hasOwnProperty(char)){
            result[s[split]] += Number(s.slice(split+1, idx));
            split = idx;
        }
        idx++;
    }
    if(result[s[split]] === undefined){
    result[s[split]] = Number(s.slice(split+1, idx-1));
    } else{
        console.log(split+'->'+s[split+1]+' : '+s[idx-1]+' == '+Number(s.slice(split+1, idx)))
        result[s[split]] += Number(s.slice(split+1, idx));
    }

    s = '';
    for (let key of Object.keys(result).sort()){
        s += `${key}${result[key]}`
        }
    

    return s;
}

console.log(rehash(str1));