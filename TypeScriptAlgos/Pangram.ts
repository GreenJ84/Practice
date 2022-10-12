//? Determine if a sentence is a pangram. 
//* A pangram (Greek: παν γράμμα, pan gramma, "every letter") is a sentence using every letter of the alphabet at least once. 
//* The best known English pangram is: 'The quick brown fox jumps over the lazy dog.'
//* The alphabet used consists of ASCII letters a to z, inclusive, and is case insensitive. Input will not contain non-ASCII symbols.

function isPangram(sentence: string): boolean {
    let sent: string[] = sentence.split('');
        sent = sent.map(item => String(item).toLowerCase());
    let pangram: boolean = true;
    const alphabet: string[] = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']
    for (let i: number = 0; i < alphabet.length; i++){
        if(sent.includes(alphabet[i])){
            continue
        }
        else { 
            pangram = false; 
        }
    }
    return pangram;
}


//! empty sentence => expect isPangram('') to === false

//! perfect lower case => expect isPangram('abcdefghijklmnopqrstuvwxyz') to === true)

//! only lower case => expect isPangram('the quick brown fox jumps over the lazy dog') to === true)

//! missing the letter 'x' => expect isPangram('a quick movement of the enemy will jeopardize five gunboats') to === false)

//! missing the letter 'h' => expect isPangram('five boxing wizards jump quickly at it') to === false)

//! with underscores => expect isPangram('the_quick_brown_fox_jumps_over_the_lazy_dog') to === true)

//! with numbers => expect isPangram('the 1 quick brown fox jumps over the 2 lazy dogs') to === true

//! missing letters replaced by numbers => expect isPangram('7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog') to === false

//! mixed case and punctuation => expect isPangram('"Five quacking Zephyrs jolt my wax bed."') to === true

//! case insensitive => expect isPangram('the quick brown fox jumps over with lazy FX') to === false