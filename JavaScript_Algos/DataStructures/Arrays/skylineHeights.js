// You are given an array with heights of consecutive buildings in the city. For example, ​[-1,7,3]​ would represent three buildings: first is actually below street level, second is seven stories high, and third is three stories high (but hidden behind the seven-story one). You are situated at street level. Return an array containing heights of the buildings you can see, in order. Given ​[1,-1,7,3]​ return ​[1,7]​.

function skylineHeights(array){
    let i = 0;
    let maxHeight = 0;
    let skyline = [];

    while ( i < array.length ){
        if (array[i] < 0){ i++; continue; }
        if ( array[i] > maxHeight ){
            skyline.push( array[i] );
            maxHeight = array[i];
        }
        i++;
    }
    console.log(skyline)
    return skyline
}

skylineHeights([1,-1,7,3,5,7,9,-11]);

