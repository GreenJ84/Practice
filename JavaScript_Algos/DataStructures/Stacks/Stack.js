/* 
- Stacks store information in the form of a list. However, - - They allow only adding and removing elements under a LIFO pattern (last in, first out). 
- Elements CANNOT be added or removed out of order, they always have to follow the LIFO pattern. 
*/

// Node Class with constructor for Stack Nodes
class StackNode {
    constructor(data){
        this.data = data;
        this.next = null;
    }
}

// Stack Class with constructor
class Stack{
    constructor(){
        this.top = null;
        this.length = 0;
    }

// Checks whether a stack is empty or not
    isEmpty(){
        if (this.top == null){
            return true;
        }
        return false;
    }

// Push (add) an Node to the END of a Stack
    push(item){
        let newNode = new StackNode(item);

        let runner = this.top; 
        this.top = newNode;
        newNode.next = runner;

        this.length++;
        return this.length;
    }

// Pop (remove) a Node fron the END of a stack
    pop(){
        if (this.isEmpty()){
            return null;
        }

        let runner = this.top.next;
        let deleted = this.top;

        this.top.next = null;
        this.top = runner;
        this.length--;

        return deleted;
    }

// Check what the top Node is
    peek(){
        return this.top;
    }

// Check what the data is in the top Node
    peekDeets(){
        return this.top.data;
    }

// Check the length of a Stack
    size(){
        return this.length;
    }

    contains( val ){
        runner = this.top
        while(runner){
            if (runner.data == val){
                return true
            }
            runner = runner.next
        }
        return false
    }
}

var stack = new Stack();