//? Given a string representing a matrix of numbers, return the rows and columns of that matrix.
// So given a string with embedded newlines like:
    // '9 8 7\n5 3 2\n6 6 7'
// Representing this matrix
    /*  9 8 7
        5 3 2
        6 6 7 */
//! your code should be able to spit out:
// -> A list of the rows, reading each row left-to-right while moving top-to-bottom across the rows,
// -> A list of the columns, reading each column top-to-bottom while moving from left-to-right.
// The rows for our example matrix:
        /*  9, 8, 7
            5, 3, 2
            6, 6, 7  */
// And its columns:
        /*  9, 5, 6
            8, 3, 6
            7, 2, 7  */

class Matrix {
    mCode: string = '';
    constructor(sCode: string) {
        this.mCode = sCode
    }
    
    get rows(): number[][] {
        // Holds multiple digits 10+
        let numString: string = '';
        // Holds each row
        let subRes: number[] = []
        // Holds result matrix
        let result: number[][] = []

        // Itterates Matrix Code
        for (let i: number = 0; i < this.mCode.length; i++){
            if (this.mCode[i] ==' '){
                // Push a numString value into a row then clearns numString
                subRes.push(parseInt(numString));
                numString = '';
            } else if ( this.mCode[i] == '\n' ) {
                // Start new list for a new row
                subRes.push(parseInt(numString));
                numString = '';

                // Pushes the row into final result and clears subRes for a new row of numbers
                result.push(subRes);
                subRes = [];
            } else {
                // Push string values into numString
                numString += this.mCode[i];
            }
        }
        // Clears any leftover numString vals into a row, the pushes the last row into the result.
        subRes.push(parseInt(numString));
        result.push(subRes);

        return result;
    }
    
    get columns(): number[][] {
        // Holds multiple digits (10+) elements
        let numString: string = '';
        // If there is more than 1 value per column
        let cont: boolean = false;
        // Holds each column starter
        let subRes: number[] = []
        // Holds result matrix
        let result: number[][] = []
        // the counter for itterating our Matric Code
        let i: number = 0

        // Goes through and sets the list for each column and first column indexes
        for ( i; i < this.mCode.length; i++){
            // Breaks the loop and flips to boolean if there is more values for each column
            if (this.mCode[i] == '\n'){ cont = true; break; }
            if (this.mCode[i] ==' '){
                // Pushes numString into a column list
                subRes.push(parseInt(numString));
                // Pushes the column list into the result matrix
                result.push(subRes);
                // Clear numString and column
                numString = '';
                subRes = [];
            } else {
                // Push string values into numString
                numString += this.mCode[i];
            }
        }
        // Pushes last first Set numString into a column list
        subRes.push(parseInt(numString));
        // Pushes last first Set column list into the result matrix
        result.push(subRes);
        // Clear numString
        numString = '';

        // Itterator for rest of matrix if available
        if (cont == true){
            // Variable to sort numbers into correct columns
            let column: number = 0;
            // Itterates Rest of Matrix Code
            for (i; i < this.mCode.length; i++){
                if (this.mCode[i] ==' '){
                    // Pushes the value into the appropriate column
                    result[column].push(parseInt(numString));
                    // Moves to the next column for the next value
                    column++;
                    // Clear numString
                    numString = '';
                } else if ( this.mCode[i] == '\n' ) {
                    // Pushes the value into the appropriate column
                    result[column].push(parseInt(numString));
                    // Reset column positioner to column 1 and clear numString
                    column = 0;
                    numString = '';
                } else {
                    // Push string values into numString
                    numString += this.mCode[i];
                }
            }
        // Clears any leftover numString vals into a column.
        result[column].push(parseInt(numString));
        }

        return result;
    }
}
