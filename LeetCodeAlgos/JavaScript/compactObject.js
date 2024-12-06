// Given an object or array obj, return a compact object.
function compactObject(obj) {
    if (obj instanceof Array) {
        return obj.filter(function (value) { return Boolean(value); })
            .map(function (value) { return typeof value === 'object' ?
            compactObject(value) : value; });
    }
    else {
        for (var _i = 0, _a = Object.keys(obj); _i < _a.length; _i++) {
            var key = _a[_i];
            if (!Boolean(obj[key])) {
                delete obj[key];
            }
            else if (typeof obj[key] === 'object') {
                obj[key] = compactObject(obj[key]);
            }
        }
    }
    return obj;
}
console.log(compactObject([null, 0, false, 1]));
console.log(compactObject({ "a": null, "b": [false, 1] }));
console.log(compactObject([null, 0, 5, [0], [false, 16]]));
