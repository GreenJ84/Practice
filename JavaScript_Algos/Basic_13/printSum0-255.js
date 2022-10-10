// Print all the integers from 0-255 along with a running sum
// Do-While loop
const do_while = () => {
let sum = 0;
let num = 0;

do{
    console.log(num)
    sum += num;
    console.log('sum is '+sum)
    num++
}
while(num < 256);

}

//While loop
const whileLoop = () => {
    let sum = 0;
    let num = 0;
    
    while(num < 256){
        console.log(num)
        sum += num;
        console.log('sum is '+sum)
        num++
    }
    
    }

//For loop
const forLoop = () => {
    let sum = 0;
    let num = 0;
    
    for (let num = 0; num < 256; num++){
        console.log(num)
        sum += num;
        console.log('sum is '+sum)
    }
}
