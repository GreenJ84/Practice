// For a game of Dungeons & Dragons, each player starts by generating a character they can play with. This character has, among other things, six abilities; strength, dexterity, constitution, intelligence, wisdom and charisma. These six abilities have scores that are determined randomly. You do this by rolling four 6-sided dice and record the sum of the largest three dice. You do this six times, once for each ability.
// Your character's initial hitpoints are 10 + your character's constitution modifier. You find your character's constitution modifier by subtracting 10 from your character's constitution, divide by 2 and round down.
//? Write a random character generator that follows the rules above.

// *For example, the six throws of four dice may look like:
    // 5, 3, 1, 6: You discard the 1 and sum 5 + 3 + 6 = 14, which you assign to strength.
    // 3, 2, 5, 3: You discard the 2 and sum 3 + 5 + 3 = 11, which you assign to dexterity.
    // 1, 1, 1, 1: You discard the 1 and sum 1 + 1 + 1 = 3, which you assign to constitution.
    // 2, 1, 6, 6: You discard the 1 and sum 2 + 6 + 6 = 14, which you assign to intelligence.
    // 3, 5, 3, 4: You discard the 3 and sum 5 + 3 + 4 = 12, which you assign to wisdom.
    // 6, 6, 6, 6: You discard the 6 and sum 6 + 6 + 6 = 18, which you assign to charisma.
    // Because constitution is 3, the constitution modifier is -4 and the hitpoints are 6.

class DnDCharacter {
    strength: number;
    dexterity: number;
    constitution: number;
    intelligence: number;
    wisdom: number;
    charisma: number;
    hitpoints: number = 10;

    constructor(){
        this.strength = DnDCharacter.generateAbilityScore();
        this.dexterity = DnDCharacter.generateAbilityScore();
        this.constitution = DnDCharacter.generateAbilityScore();
        this.intelligence = DnDCharacter.generateAbilityScore();
        this.wisdom = DnDCharacter.generateAbilityScore();
        this.charisma = DnDCharacter.generateAbilityScore();
        this.hitpoints += DnDCharacter.getModifierFor(this.constitution);
    }
    public static generateAbilityScore(): number {
        let rolls: number[] = [];
        let i: number = 0;
        while (i < 4){ 
            rolls.push(Math.round(Math.random()*6));
            i++;
        }
        let min: number = rolls[0];
        let sum: number = 0;
        for (let j in rolls){
            sum += rolls[j];
            rolls[j] < min ? min = rolls[j] : ''
        }
        return sum - min;
    }
    
    public static getModifierFor(abilityValue: number): number {
        return Math.floor((abilityValue-10)/2)
    }

}
const character = new DnDCharacter();
console.log(character.strength, character.wisdom, character.intelligence, character.dexterity, character.constitution, character.charisma, character.hitpoints)

//! expect DnDCharacter.getModifierFor(3)) to equal -> -4
//! expect DnDCharacter.getModifierFor(4)) to equal -> -3
//! expect DnDCharacter.getModifierFor(5)) to equal -> -3
//! expect DnDCharacter.getModifierFor(6)) to equal -> -2
//! expect DnDCharacter.getModifierFor(7)) to equal -> -2
//! expect DnDCharacter.getModifierFor(8)) to equal -> -1
//! expect DnDCharacter.getModifierFor(9)) to equal -> -1
//! expect DnDCharacter.getModifierFor(10) to equal -> 0
//! expect DnDCharacter.getModifierFor(11) to equal -> 0
//! expect DnDCharacter.getModifierFor(12) to equal -> 1
//! expect DnDCharacter.getModifierFor(13) to equal -> 1
//! expect DnDCharacter.getModifierFor(14) to equal -> 2
//! expect DnDCharacter.getModifierFor(15) to equal -> 2
//! expect DnDCharacter.getModifierFor(16) to equal -> 3
//! expect DnDCharacter.getModifierFor(17) to equal -> 3
//! expect DnDCharacter.getModifierFor(18) to equal -> 4

//! DnDCharacter.generateAbilityScore() is supposed to return a number greater than or equal to 3 and less than or equal to 18

//! character.strength is supposed to be greater than or equal to 3 and less than or equal to 18
//! character.dexterity is supposed to be greater than or equal to 3 and less than or equal to 18
//! character.constitution is supposed to be greatthan or Or equal to 3 and less than or equal to 18
//! character.intelligence is supposed to be greatthan or Or equal to 3 and less than or equal to 18
//! character.wisdom is supposed to be greater Orthan oequal to 3 and less than or equal to 18
//! character.charisma is supposed to be greater than or equal to 3 and less than or equal to 18
//! Each ability is only calculated once