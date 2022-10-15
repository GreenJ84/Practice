// Implement ​generateCoinChange(cents)​ that accepts a parameter for the number of cents, and computes how to represent that amount with the smallest number of coins. Console.log the result.


function generateCoinChange(cents){
    let quarters = 0;
    let dimes = 0;
    let nickles = 0;
    while(cents - 25 > 0){
        cents -= 25;
        quarters++;
    }
    while(cents - 10 > 0){
        cents -= 10;
        dimes++;
    }
    while(cents - 5 > 0){
        cents -= 5;
        nickles++;
    }
    console.log('Qaurters: '+quarters+'\nDimes: '+dimes+'\nNickles: '+nickles+'\nPennies: '+cents);
}

generateCoinChange(26);