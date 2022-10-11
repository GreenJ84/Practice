// Print all odd integers from 1 to 255.

function allOdd(){
    for (let i = 0; i < 256; i++){
        if (i % 2 == 1){
            console.log(i)
        }
    }
}

function allOdd2(){
    let i = 0;
    while(i < 256){
        i % 2 == 1 ? console.log(i) : '';
        i++;
    }

}

allOdd2();