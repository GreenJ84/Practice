// Create a function that, given an input string, returns a boolean whether parentheses in that string are valid. Given input ​"y(3(p)p(3)r)s"​, return true. Given ​"n(0(p)3"​, return ​false​. Given ​"n)0(t(0)k"​, return ​false​.

function parensValid(str){
    let openP = 0;
    str.split('').forEach((item) => {
        if (item === '('){ openP++ }
        else if(item === ')'){ openP--;}
    })
    return openP === 0 ? true : false
}

console.log(parensValid("y(3(p)p(3)r)s"));
console.log(parensValid("n(0(p)3"));

