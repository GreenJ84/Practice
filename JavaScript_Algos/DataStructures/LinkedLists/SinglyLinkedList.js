// Create a class for each node within the list
class ListNode {
    // Each node has two properties
    constructor( data ) {
// Its value
        this.data = data;
// A pointer that indicates the node that follows
        this.next = null;
    }
}

// Create a class for the list
class SinglyLinkedList {

// The list has a head properties by default
    constructor() {

        this.head = null;
    }

//* Checks on the SLL
//===========================================================
// Checks whether a SLL is empty
isEmpty() {
    if ( this.head ) {
        return false;
    }
    return true;
}

// Checks and returns the head of a list
getHead(){
    if ( this.head ) {
        return this.head.data;
    }
    return null;
}

getTail(){

}

// Check if a Node containing certain data exists
ifExists(searchTerm){
    let runner = this.head;
    if (this.isEmpty()){
        return ("Nothing to check against!");
    }
    while (runner.next){
        if (runner.data == searchTerm){
            return true;
        }
        runner = runner.next;
    }
    return false;
}

// Checks and returns the second to last Node
secondToLast(){
    if (this.head == null || this.head.next == null){
        return null;
    }
    let runner = this.head;
    while (runner.next != null){
        if (runner.next.next == null){
            console.log(runner);
            return runner;
        }
        runner = runner.next;
    }
}

// Checks and returns the length of a linked list
length(){
    if ( this.isEmpty() ) { return null }
    let runner = this.head;
    let len = 0;
    while ( runner ){
        len++;
        runner = runner.next
    }
    console.log(len);
    return len;
}

// Checks and returns the average value of the list data
average(){
    if ( this.isEmpty()){ return null }
    let runner = this.head;
    let avg = 0;
    while ( runner ){
        avg += runner.data;
        runner = runner.next
    }
    avg /= this.length();
    console.log(avg);
    return avg;
}

min(){
    if ( this.isEmpty()){ return null}
    let runner = this.head;
    let min = this.head.data;
    while ( runner ){
        runner.data < min ? min = runner.data: '';
        runner = runner.next
    }
    console.log(min);
    return min;
}

max(){
    if ( this.isEmpty()){ return null}
    let runner = this.head;
    let max = this.head.data;
    while ( runner ){
        runner.data > max ? max = runner.data: '';
        runner = runner.next
    }
    console.log(max);
    return max;
}

// Checks whether a list has a loop within itself
hasLoop(){ 
    if (this.isEmpty() || this.head.next == null){
        return false;
    }
    let runner1 = this.head;
    let runner2;
    let runner3;
    let runner4;
    let runner5;
    let runner6;
    if (this.head.next){
        runner2 = this.head.next;
    }
    if (this.head.next.next){
        runner3 = this.head.next.next;
    }
    if (this.head.next.next.next){
        runner4 = this.head.next.next.next;
    }
    if (this.head.next.next.next.next){
        runner5 = this.head.next.next.next.next;
    }
    if (this.head.next.next.next.next.next){
        runner6 = this.head.next.next.next.next.next;
    }

    while (runner1){
        if (runner1 == runner2 || runner1 == runner3 || runner1 == runner4 || runner1 == runner5 || runner1 == runner6){
            return true;
        }
        if (runner2 == runner3 || runner2 == runner4 || runner2 == runner5 || runner2 == runner6){
            return true;
        }
        if (runner3 == runner4 || runner3 == runner5 || runner3 == runner6){
            return true;
        }
        if (runner4 == runner5 || runner4 == runner6){
            return true;
        }
        if (runner4 == runner6){
            return true;
        }

        if (runner1.next != null){
        runner1 = runner1.next;
        } else { return false;}

        if (runner2.next.next != null){
        runner2 = runner2.next.next;
        } else { return false;}

        if (runner3.next.next.next != null){
        runner3 = runner3.next.next.next;
        } else { return false;}

        if (runner4.next.next.next.next != null){
        runner4 = runner4.next.next.next.next;
        } else { return false;}

        if (runner5.next.next.next.next.next != null){
        runner5 = runner5.next.next.next.next.next;
        } else { return false;}

        if (runner6.next.next.next.next.next.next != null){
        runner6 = runner6.next.next.next.next.next.next;
        } else { return false;}
    }
    return false;
    // setInterval(noLoop(), 1000);
}
//* Adding to the SLL
//===========================================================

// Insert Node at the front of the list
insertAtFront( data ) {
    let newNode = new ListNode( data );
    newNode.next = this.head;
    this.head = newNode;
    return;
}

// Insert a Node at the end of a list
insertAtBack( data ) {
    let newNode = new ListNode( data );
    if ( this.isEmpty() ){
        this.head = newNode;
    } else {
        let runner = this.head;
        while( runner.next ){
            runner = runner.next;
        }
        runner.next = newNode;
    }
    return newNode;
}

// Inserts a Node before a Node with the provided target value if such Node excists
insertBeforeVal(newVal, targetValue){
    let newNode = new ListNode(newVal);
    if (this.head.data == targetValue){
        let temp = this.head;
        this.head = newNode;
        newNode.next = temp;
    }
    let runner = this.head;
    while( runner.next != null){
        if (runner.next.data == targetValue){
            let temp = runner.next;
            runner.next = newNode;
            runner.next.next = temp;
            return true;
        }
        runner = runner.next;
    }
    return false;
}



//* Removing from the SLL
//===========================================================

// Remove a Node from the front of a list
removeFromFront() {
    if( !this.isEmpty()){
        this.head = this.head.next;
    }
    return;
}

// Remove a Node form the end of a list
removeFromBack(){
    let runner = this.head;
    let deleted;
    if (this.isEmpty){
        return ("No values to delte!");
    }
    if (this.head.next == null){
        deleted = this.head;
        this.head = null;
        return deleted;
    }
    else if(this.head.next.next == null){
        deleted = this.head.next;
        this.head.next = null;
        return deleted;
    }
    while (runner.next.next){
        runner = runner.next;
        console.log(runner);
    }
    deleted = runner.next;
    console.log(deleted);
    runner.next = null;
    this.display();
    return deleted;
}

// Remove a Node if it contains certain data
removeVal(value){
    let removedValue = false;
    let reomveCounter = 0;
    if (this.head == null){
        return removedValue;
    }
    if (this.head.data == value){
        this.head = this.head.next;
        removedValue = true;
        reomveCounter++;
    }
    let runner = this.head;
    while (runner.next != null){
        if (runner.next.data == value){
                runner.next = runner.next.next;
            removedValue = true;
            reomveCounter++;
        } else{
        runner = runner.next;
        }
    }
    console.log(reomveCounter);
    return removedValue;
}


//* SLL Functions
//===========================================================

// Reverse the order of Nodes in a List
    reverse(){
        if (this.isEmpty() || this.head.next == null){
            return this;
        }

        let runner1 = this.head;
        let runner2 = this.head.next;
        let temp;

        this.head = null;
        runner1.next = null;

        while(runner2.next){
            temp = runner2.next;
            runner2.next = runner1;
            runner1 = runner2;
            runner2 = temp;
        }
        runner2.next = runner1;
        this.head = runner2;
        return this;
    }

// Display the list in output
display(){
    if( this.isEmpty() ){
        console.log("This list is empty!")
    } else {
        let n = 0;
        let runner = this.head;
        console.log('node ' + n + ': '+runner.data);
        while( runner.next ){
            runner = runner.next;
            n++;
            console.log('node ' + n + ': '+runner.data);
        }

    }
    return;
}

// Move the smallest Node to the front of the list
moveMinToFront(){
    let runner = this.head;
    let counter=0;
    let index;
    let min = this.head;

    while (runner.next != null){
        if (runner.next.data < min.data){
            min = runner.next;
            index = counter;
        }
        runner = runner.next;
        counter++;
    }
    if (min == this.head){
        return list;
    }

    runner = this.head;
    for (index; index > 0; index--){
        runner = runner.next
    }
    let temp1 = runner.next;
    if (runner.next.next != null){
        runner.next = runner.next.next;
    } else {
        runner.next = null;
    }

    let temp = this.head;
    this.head = temp1;
    temp1.next = temp;
    return list;
}

// Concatonate a new list onto the back of this list
    concat(concatList){
        if (this.isEmpty()){
            this.head = concatList.head;
            return list;
        }
        
        let runner1 = this.head;
        while (runner1.next != null){
            runner1 = runner1.next;
        }
        runner1.next = concatList.head;
        return list;
    }

// Locates the first node with value and move all nodes ​less than​ value to front, and nodes ​greater than value to be later. 
partition( value ){
    if (this.isEmpty()){ return null; }
    let runner = this.head
}

// Remove nodes with duplicate values.
dedupeList(){
    if (this.isEmpty()){ return null; }
    let runner = this.head
    let datas = [this.head.data]
    while ( runner.next ){
        if (datas.includes(runner.next.data)){
            let temp = runner.next
            runner.next = temp.next || null
            temp.next = null
            }
        else{
            runner = runner.next
            datas.push(runner.data)
        }
        }
    }


// Split a list on a Node that contains data provided and return the new created list
splitOnVal(val){
    if (this.isEmpty()){
        return null;
    }
    if (this.head.data == val){
        return list;
    }

    let runner = this.head;
    let secondHead = null;

    while (runner.next != null){
        if (runner.next.data == val){
            secondHead = runner.next;
            runner.next = null;
            break;
        }
        runner = runner.next;
    }

    if (secondHead != null){
        let newList = new SinglyLinkedList();
        newList.head = secondHead;
        let runner2 = secondHead;
        while (runner2.next != null){
            secondHead.next = runner2.next;
            secondHead = secondHead.next;
            runner2 = runner2.next;
        }
        newList.display();
        return newList;
    }

    return null;
}
}

//? Use the SinglyLinkedList here
var list = new SinglyLinkedList();
list.insertAtFront(2);
list.insertAtFront(4);
list.insertAtFront(4);
list.insertAtFront(4);
list.insertAtFront(6);
list.insertAtFront(8);
list.insertAtFront(8);
list.insertAtFront(8);
list.dedupeList()
list.display()