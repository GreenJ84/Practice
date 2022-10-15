// Implement a function ​sumToOne(num)​ that, given a number, sums that number’s digits repeatedly until the sum is only one digit. Return thatfinalonedigitresult.

function sumToOne(num){
    let sum = 0;
    while(num > 9){
        let temp = num;
        while (temp > 9){
            sum += temp % 10;
            temp = Math.floor(temp / 10);
        }
        sum += temp;
        num = sum;
        sum = 0;
    }
    return num;
}

console.log(sumToOne(99999999999))