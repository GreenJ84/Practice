//? Create a class to Organize a Grade School with a student roster
//! You should be able to:
//? Given students' names along with the grade that they are in, add the student to the roster for the school.
//? Get a list of all students enrolled in a grade
//? Get a sorted list of all students in all grades. Grades should sort as 1, 2, 3, etc., and students within a grade should be sorted alphabetically by name.
// Note that all our students in our town have unique names. (It's a small town, what do you want?)



const deepClone = (obj: {}) => JSON.parse(JSON.stringify(obj))

export class GradeSchool {
    #sRoster: {[grade: number]: string[] };
    constructor(){
        this.#sRoster = {};
    }

    roster() {
        return deepClone(this.#sRoster);
    }

    add(name: string, grade: number ) {
        Object.keys(this.#sRoster).forEach((value: string) => {
        this.#sRoster[parseInt(value)] = this.#sRoster[parseInt(value)].filter((alumn: string) => alumn !== name)
        });
        if (this.#sRoster[grade]){
            this.#sRoster[grade].push(name);
            this.#sRoster[grade].sort()
        } else {
            this.#sRoster[grade] = [name]
        }
    }

    grade(grade: number) {
        if (this.#sRoster[grade]){
            let shell: string[] = this.#sRoster[grade];
            shell.sort();
            return [...shell]
        } else{
            return []
        }
    }
}



// ================ Tests ===========

// 'a new school has an empty roster'
//! expect school.roster() to Equal {}

// 'adding a student adds them to the roster for the given grade'
    //school.add('Aimee', 2)
//! expect school.roster() to Equal { 2: ['Aimee'] }

// 'adding more students to the same grade adds them to the roster'
    // school.add('Blair', 2)
    // school.add('James', 2)
    // school.add('Paul', 2)
//! expect school.roster() to Equal { 2: ['Blair', 'James', 'Paul'] }

// 'adding students to different grades adds them to the roster'
    // school.add('Chelsea', 3)
    // school.add('Logan', 7)
//! expect school.roster() to Equal { 3: ['Chelsea'], 7: ['Logan'] }

// 'grade returns the students in that grade in alphabetical order'
    // school.add('Franklin', 5)
    // school.add('Bradley', 5)
    // school.add('Jeff', 1)
//! expect school.grade(5) to Equal ['Bradley', 'Franklin']

// 'grade returns an empty array if there are no students in that grade'
//! expect school.grade(1) to Equal []

// 'the students names in each grade in the roster are sorted'
    // school.add('Jennifer', 4)
    // school.add('Kareem', 6)
    // school.add('Christopher', 4)
    // school.add('Kyle', 3)
    const expectedSortedStudents = {
        3: ['Kyle'],
        4: ['Christopher', 'Jennifer'],
        6: ['Kareem'],
    }
//! expect school.roster() to Equal { 3: ['Kyle'], 4: ['Christopher', 'Jennifer'], 6: ['Kareem'], }

// 'roster cannot be modified outside of module'
    // school.add('Aimee', 2)
    // const roster = school.roster()
    // roster[2].push('Oops.')
//! expect school.roster() to Equal { 2: ['Aimee'] }

// 'roster cannot be modified outside of module using grade()'
    // school.add('Aimee', 2)
    // school.grade(2).push('Oops.')
//! expect school.roster() to Equal { 2: ['Aimee'] }

// "a student can't be in two different grades"
    // school.add('Aimee', 2)
    // school.add('Aimee', 1)
//! expect school.grade(2) to Equal []