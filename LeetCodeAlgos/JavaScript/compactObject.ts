// Given an object or array obj, return a compact object.

// A compact object is the same as the original object, except with keys containing falsy values removed. This operation applies to the object and any nested objects. Arrays are considered objects where the indices are keys. A value is considered falsy when Boolean(value) returns false.

// You may assume the obj is the output of JSON.parse. In other words, it is valid JSON.

type JSONValue = null | boolean | number | string | JSONValue[] | { [key: string]: JSONValue };
type Obj = Record<string, JSONValue> | Array<JSONValue>;

function compactObject(obj: Obj | JSONValue[] | { [key: string]: JSONValue }): Obj {
    if (obj instanceof Array) {
      return obj.filter(value => Boolean(value))
        .map(value => typeof value === 'object' ?
          compactObject(value!) : value
        );
    } else {
      for (const key of Object.keys(obj)) {
        if (!Boolean(obj[key])) {
          delete obj[key];
        } else if (typeof obj[key] === 'object') {
          obj[key] = compactObject(obj[key] as Obj);
        }
      }
    }
    return obj;
}

console.log(compactObject([null, 0, false, 1]));
console.log(compactObject({"a": null, "b": [false, 1]}));
console.log(compactObject([null, 0, 5, [0], [false, 16]]));
