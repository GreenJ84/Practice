// Implement a ‘die’ that randomly returns an integer between 1 and 6 inclusive. Roll a pair of these dice, tracking the statistics until doubles are rolled. Display the ​number of rolls​, ​min,​ ​max,​ and ​average​.

function rollDice(){
    return Math.ceil(Math.random()*6)
}

function doublesStats(){
    let numRolls = 0, min = 12, max = 0, avg = 0;
    let roll1 = 0;
    let roll2 = 1;

    while(roll1 != roll2){
        roll1 = rollDice();
        roll2 = rollDice();

        if (roll1 + roll2 > max){ max = roll1 + roll2 }
        if (roll1 + roll2 < min){ min = roll1 + roll2 }
        numRolls += 1;
        avg += roll1 + roll2;
    }
    avg /= numRolls;

    console.log('Stats are:\nExited Double: '+roll1+"s\nNumber of Rolls: "+numRolls+'\nMinimum roll: '+min+'\nMaximum Roll: '+max+'\nAverage Roll: '+avg);
}

doublesStats();