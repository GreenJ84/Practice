// Replicate JavaScript’s ​concat()​. Create a standalone function that accepts two arrays. Return a ​new​ array containing the first array’s elements, followed by the second array’s elements. Do not alter the original arrays. Ex.: arrConcat( ['a','b'], [1,2] )​ should return ​['a','b',1,2]​.

function arrConcat(array1, array2){
    let concat = [];
    for (let x in array1){
        concat.push(array1[x]);
    }
    for (let x in array2){
        concat.push(array2[x]);
    }
    console.log(concat);
    return concat
};

arrConcat([1,2,3], [4,5,6]);
arrConcat([1,2,3], ['a', 'b', 'c']);
