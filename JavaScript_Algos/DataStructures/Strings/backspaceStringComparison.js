
// Given two strings S and T containing only lowercase letters and "#" characters,return if they are equal when both are typed into empty text editors.
// # character means a backspace character.i.e., after processing the backspaces, are the two strings equal?


const S1 = "ac";
const T1 = "ac";
const expected1 = true;
// Explanation: Both S and T become "ac"

const S2 = "";
const T2 = "";
const expected2 = true;
// Explanation: Both S and T become ""

const S3 = "a##c";
const T3 = "#a#c";
const expected3 = true;
// Explanation: Both S and T become "c"

const S4 = "a#c";
const T4 = "b";
const expected4 = false;
// Explanation: S becomes "c" while T becomes "b".


// Determines if the given strings are equal after the backspace (#) characters

function backspaceStringCompare(S, T) {
    let i = 0;
    for (i; i < S.length; i++){
        if (S[i] === '#'){
            S = S.replace(/(#)|([a-z]#)/, '')
            i = -1;
        }
    }
    for (i = 0; i < T.length; i++){
        if (T[i] === '#'){
            T = T.replace(/(#)|([a-z]#)/, '') 
            i = -1
        }
    }

    return S === T ? true: false
}
console.log(backspaceStringCompare(S1, T1));
console.log(backspaceStringCompare(S2, T2));
console.log(backspaceStringCompare(S3, T3));
console.log(backspaceStringCompare(S4, T4));