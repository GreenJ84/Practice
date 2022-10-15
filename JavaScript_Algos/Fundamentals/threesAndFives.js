//Create function ​ThreesFives()​ that adds each value from 100 and 4000000 (inclusive) if that value is evenly divisible by 3 or 5 ​but not both​. Display the final sum in the console.
//Second:​ Change your function to make a ​BetterThreesFives(start,end)​where ​start​ and ​end values are customizable. You can think of the above ​ThreesFives()​ function as BetterThreesFives(100,4000000)​.

function ThreesFives(){
    let i = 100;
    let sum = 0;
    while (i < 4000000){
        if (i % 3 == 0 || i % 5 == 0){
            if (i % 3 == 0 && i % 5 == 0){
                // console.log('Not adding'+i);
            } else {
                sum += i;
            }
        }
        i++
    }
    console.log(sum);
}

ThreesFives()

function BetterThreesFives(start, end){
    let i = start;
    let sum = 0;
    while (i < end){
        if (i % 3 == 0 || i % 5 == 0){
            if (i % 3 == 0 && i % 5 == 0){
                // console.log('Not adding'+i);
            } else {
                sum += i;
            }
        }
        i++
    }
    console.log(sum);
}

BetterThreesFives(100,4000000)