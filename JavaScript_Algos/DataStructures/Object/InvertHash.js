// Create invertHash(assocArr) that converts a hash’s keys to values and values to corresponding keys. Example: given ​{"name": "Zaphod"; "numHeads": 2}​, return ​{"Zaphod": "name"; 2: "numHeads"}​. You will need to learn and use a JavaScript ​for ... in​ h​ ere!

function invertHash(map){
    let values = Object.keys(map);
    let keys = Object.values(map);
    map = {};
    for ( let i = 0; i < values.length; i++){
        map[keys[i]] = values[i];
    }
    console.log(map);
    return map;
}

invertHash({1: 'a', 2: 'b', 3: 'c'});