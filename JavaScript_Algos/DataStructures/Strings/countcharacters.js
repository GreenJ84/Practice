
//* Function to take in any string and count the number of times that character is present

const check1 = 'How many letters are here'

function count (string) {  
    let i = 0;
    let count = {};
    while(i < string.length){
        if (string[i] in count){
        count[string[i]] += 1;
        } else {
        count[string[i]] = 1;
        }
        i++;
    }
    
    return count;
    }

console.log(count(check1));