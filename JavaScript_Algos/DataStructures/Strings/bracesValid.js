// Given a string, returns whether the sequence of various parentheses, braces and brackets within it are valid. For example, given the input string "w(a{t}s[o(n{c}o)m]e)h[e{r}e]!"​, return true​. Given ​"d(i{a}l[t]o)n{e"​, return false​. Given ​"a(1)s[O(n]0{t)0}k"​, return false​.

function invalid(response, current){
    return response === current ? false : true;
}

function bracesValid(str){
    let bStart = {
        '(': 0,
        '{': 0,
        '[': 0 }
    let open = [];
    let inValid = false;
    let paren = '(', curl = '{', square = '[';
    str.split('').forEach((item) => {
        if (item === paren){ bStart[paren]++; open.push(paren); }
        else if(item === ')'){ 
            bStart[paren]--;
            if (invalid(open.pop(), paren)){ inValid = true; }
            }
        else if (item === curl){ bStart[curl]++; open.push(curl)}
        else if(item === '}'){ 
            bStart[curl]--;
            if (invalid(open.pop(), curl)){ inValid = true; }
            }
        else if (item === square){ bStart[square]++; open.push(square) }
        else if(item === ']'){ 
            bStart[square]--;
            if (invalid(open.pop(), square)){ inValid = true; }
            }
    })
    return bStart[paren] === 0 && bStart[curl] === 0 && bStart[square] === 0 && invalid === false ? true : false
}

console.log(bracesValid("w(a{t}s[o(n{c}o)m]e)h[e{r}e]!"));
console.log(bracesValid("d(i{a}l[t]o)n{e"));
console.log(bracesValid("a(1)s[O(n]0{t)0}k"));
