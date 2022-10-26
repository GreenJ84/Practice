// Given an array of objects that contain a category key, return a hash table to group the objects by their category.
// Make the grouping case-insensitive.
// Bonus: allow the key that is grouped by to be provided.

const objects = [
    {
        name: "Baby Yoda",
        category: "cute",
    },
    {
        name: "Cricket Protein",
        category: "food",
    },
    {
        name: "Shibe",
        category: "Cute",
    },
    {
        name: "Ancient India",
        category: "Cradle of Civilization",
    },
    {
        name: "Wasp Crackers",
        category: "Food",
    },
    {
        name: "The Fertile Crescent",
        category: "Cradle of Civilization",
    },
];

const expected = {
    cute: [
        {
        name: "Baby Yoda",
        category: "cute",
        },
        {
        name: "Shibe",
        category: "Cute",
        },
    ],
    food: [
        {
        name: "Cricket Protein",
        category: "food",
        },
        {
        name: "Wasp Crackers",
        category: "Food",
        },
    ],
    "cradle of civilization": [
        {
        name: "Ancient India",
        category: "Cradle of Civilization",
        },
        {
        name: "The Fertile Crescent",
        category: "Cradle of Civilization",
        },
    ],
};

//! Creates a hash table of case-insensitive categories mapped to the items belonging to that category.
function groupObjects(items) {
    let hash = {};

    for (let item of items){
        let catg = item.category.toLowerCase();
        if (catg in hash){
            hash[catg].push(item)
        } else {
            hash[catg] = [item]
        }
    }
    return hash;
}
console.log(groupObjects(objects));