// Number of connected Components in an Undirected Graph

function connectedComponents(n, edges){
    let connections = 0;
    let i = 0;
    let j = 0;

    while (i < n){
        let first = edges[j][0];
        let second = edges[j][1];

        if (parseInt(first) === i && parseInt(second) === i+1){ connections += 1; }
        while (parseInt(first) === i && parseInt(second) === i+1){
            i++; j++;
            if (i === n || j === edges.length){ break }
            first = edges[j][0];
            second = edges[j][1];
        }

        i++;
    }
    return connections;
}

console.log(connectedComponents(5, [[0,1], [1,2], [3,4]]));
console.log(connectedComponents(5, [[0,1], [1,2], [2,3], [3,4]]));
