// Number of connected Components in an Undirected Graph
function connectedComponents(n, edges){
    let connections = 1;
    let i = 0;
    let j = 0;

    while (i < n-1){
        let found = false;
        while (j < edges.length){
            if (edges[j][0] === i && edges[j][1] === i+1){
                found = true;
                edges.splice(j, 1);
                j = 0;
                break;
            }
            j++
        }
        i++;
        if (found != true){ connections += 1; j = 0; }
    }
    return connections;
}

console.log(connectedComponents(5, [[0,1], [1,2], [3,4]]));
console.log(connectedComponents(5, [[0,1], [1,2], [2,3], [3,4]]));


// const array = [[1,2], [4,5], [7,9]]
// console.log(array.includes([5]));
// console.log(array.includes([1,2]));
// console.log(array.includes([7,9]));