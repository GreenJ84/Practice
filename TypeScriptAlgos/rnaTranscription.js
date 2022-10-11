//? Given a DNA strand, return its RNA complement (per RNA transcription).
// Both DNA and RNA strands are a sequence of nucleotides.
// The four nucleotides found in DNA are adenine (A), cytosine (C), guanine (G) and thymine (T).
// The four nucleotides found in RNA are adenine (A), cytosine (C), guanine (G) and uracil (U).
// Given a DNA strand, its transcribed RNA strand is formed by replacing each nucleotide with its complement:
// G -> C | C -> G | T -> A | A -> U
function toRna(dna) {
    var rna = '';
    for (var x = 0; x < dna.length; x++) {
        if (dna[x] === 'G') {
            rna += 'C';
        }
        else if (dna[x] === 'C') {
            rna += 'G';
        }
        else if (dna[x] === 'T') {
            rna += 'A';
        }
        else if (dna[x] === 'A') {
            rna += 'U';
        }
        else {
            throw new Error('Invalid input DNA.');
        }
    }
    return rna;
}
// transcribes cytosine to guanine'
toRna('C'); // 'G'
// transcribes guanine to cytosine'
toRna('G'); // 'C'
// transcribes adenine to uracil'
toRna('A'); // 'U'
// transcribes thymine to adenine'
toRna('T'); // 'A'
// transcribes all dna nucleotides to their rna complements'
toRna('ACGTGGTCTTAA'); // 'UGCACCAGAAUU')
toRna('U'); // Throw Error -> 'Invalid input DNA.'
toRna('XXX'); // Throw Error -> 'Invalid input DNA.'
toRna('ACGTXXXCTTAA'); // Throw Error -> 'Invalid input DNA.'
