// Write a function that is given an array and each time the value is "food" it should console log "yummy". If "food" was not present in the array console log "I'm hungry" once.

function alwaysHungry(arr) {
    let yummies = false;
    for (item in arr){
        if (arr[item] === 'food'){
            console.log('Yummy');
            yummies = true;
        }
    }
    yummies === true ? '' : console.log("I'm hungry");
}

alwaysHungry([3.14, "food", "pie", true, "food"]);
// this should console log "yummy", "yummy"

alwaysHungry([4, 1, 5, 7, 2]);
// this should console log "I'm hungry"