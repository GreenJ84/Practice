// Write a function that will reverse the values an array and return them

function reverse(arr) {
    let reverse = []
    for (let i = arr.length-1; i > -1; i--){
        reverse.push(arr[i]);
    }
    return reverse;
}

var result = reverse(["a", "b", "c", "d", "e"]);
console.log(result); // we expect back ["e", "d", "c", "b", "a"]