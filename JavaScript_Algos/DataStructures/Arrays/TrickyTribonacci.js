// Why stop with fibonacci? Create a function to retrieve a “tribonacci” number, from the sum of the previous ​3​. Tribonaccis are {0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81, ...}. Again, use a time-space tradeoff to make this fast.

function tribbonacci(n){
    if (n < 0){
        return null;
    }
    else if (n <= 1){
        return 0;
    } 
    else if (n > 1 && n < 4 ){
        return 1
    }
    else{
        let i = 4
        first = 0;
        second = 1;
        third = 1;
        while (i <= n){
            let temp = first + second + third;
            first = second;
            second = third;
            third = temp;
            i++;
        }
        console.log(third);
        return third;
    }
}

tribbonacci(5);
tribbonacci(6);
tribbonacci(7);
tribbonacci(10);