// Given two arrays, create an associative array (map) containing keys of the first, and values of the second. For ​arr1 = ["abc", 3, "yo"]​ and ​arr2 = [42, "wassup", true]​, return ​{"abc": 42, 3: "wassup", "yo": true}​.

function arr2Map(array1, array2){
    if (array1.length != array2.length){
        throw new Error('Array lengths incompatable for Map conversion');
    }
    let results = {};
    for (let i = 0; i < array1.length; i++){
        results[array1[i]]= array2[i]
    }
    console.log(results);
    return results;
}

arr2Map([1,2,3], ['a', 'b', 'c']);
arr2Map([1,2,3], ['alpha', 'beta', 'canoli']);
