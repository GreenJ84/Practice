//? Bob is a lackadaisical teenager. In conversation, his responses are very limited. Write a function that represents his attitude (responses) when passed statements.
// Bob answers 'Sure.' if you ask him a question, such as "How are you?".
// He answers 'Whoa, chill out!' if you YELL AT HIM (in all capitals).
// He answers 'Calm down, I know what I'm doing!' if you yell a question at him.
// He says 'Fine. Be that way!' if you address him without actually saying anything.
// He answers 'Whatever.' to anything else.
// Bob's conversational partner is a purist when it comes to written communication and always follows normal rules regarding sentence punctuation in English.
function hey(message) {
    // Remove WhiteSpace
    message = message.replace(/\s/g, '');
    // Check if there was nothing said
    if (message.length < 1) {
        return 'Fine. Be that way!';
    }
    // Check if there is an all caps question.
    if (message == message.toUpperCase() && message[message.length - 1] == '?') {
        // Make sure there are characters for a statement (Numbers not included in a 'statement')
        if (message.match((/[a-zA-Z]+/g))) {
            return "Calm down, I know what I'm doing!";
        }
    }
    // Check if it's ann all caps statement
    if (message == message.toUpperCase()) {
        // Make sure there are characters for a statement (Numbers not included in a 'statement')
        if (message.match((/[a-zA-Z]+/g))) {
            return 'Whoa, chill out!';
        }
    }
    // Checks if it was a question
    if (message[message.length - 1] == '?') {
        return 'Sure.';
    }
    // Return for everything else
    return 'Whatever.';
}
//! hey('Tom-ay-to, tom-aaaah-to.') should return => 'Whatever.'
//! hey('WATCH OUT!') should return => 'Whoa, chill out!'
//! hey('FCECDFCAAB') should return => 'Whoa, chill out!'
//! hey('Does this cryogenic chamber make me look fat?') should return => 'Sure.'
//! hey('You are, what, like 15?') should return => 'Sure.'
//! hey('fffbbcbeab?') should return => 'Sure.'
//! hey("Let's go make out behind the gym!") should return => 'Whatever.'
//! hey("It's OK if you don't want to go to the DMV.") should return => 'Whatever.'
//! hey('WHAT THE HELL WERE YOU THINKING?') should return => "Calm down, I know what I'm doing!"
//! hey('1, 2, 3 GO!') should return => 'Whoa, chill out!'
//! hey('1, 2, 3') should return => 'Whatever.'
//! hey('4?') should return => 'Sure.'
//! hey('ZOMG THE %^*@#$(*^ ZOMBIES ARE COMING!!11!!1!') should return => 'Whoa, chill out!'
//! hey('I HATE THE DMV') should return => 'Whoa, chill out!'
//! hey('Ending with ? means a question.') should return => 'Whatever.'
//! hey('Wait! Hang on.  Are you going to be OK?') should return => 'Sure.'
//! hey('') should return => 'Fine. Be that way!'
//! hey('   ') should return => 'Fine. Be that way!'
//! hey('\t\t\t\t\t\t\t\t\t\t') should return => 'Fine. Be that way!'
//! hey('\nDoes this cryogenic chamber make me look fat?\nNo.') should return => 'Whatever.'
//! hey('         hmmmmmmm...') should return => 'Whatever.'
//! hey('Okay if like my  spacebar  quite a bit?   ') should return => 'Sure.'
//! hey('\n\r \t') should return => 'Fine. Be that way!'
//! hey('This is a statement ending with whitespace      ') should return => 'Whatever.'
