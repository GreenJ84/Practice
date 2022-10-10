class QueueNode{
    constructor(data){
        this.data = data;
        this.next = null;
    }

}

class StackNode {
    constructor(data){
        this.data = data;
        this.next = null;
    }
}

class Stack{

    constructor(){
        this.top = null;
        this.length = 0;
    }

    push(item){
        let newNode = new StackNode(item);

        let runner = this.top; 
        this.top = newNode;
        newNode.next = runner;

        this.length++;
        return this.length;
    }
}

class Queue{
    constructor(){
        this.head = null;
        this.tail = null;
        this.size = 0;
    }

    isEmpty(){
        if (this.head == null){
            return true;
        }
        return false;
    }

    enqueue(val){
        let newNode = new QueueNode(val);

        if (this.head == null){
            this.head = newNode;
            this.tail = newNode;
            this.size++;
        } else{

        this.tail.next = newNode;
        this.tail = this.tail.next;
        this.size++;
        }

        return this.size;

    }

    dequeue(){

        if (this.head == null){
            return null;
        }
        if (this.size == 1){
            this.head = null;
            this.tail = null;
        } 
        let oldHead = this.head;

        this.head = this.head.next;
        this.size--;

        return oldHead;
    }

    dequeueVal(erraseVal){
        let runner = this.head;
        while(runner.next){
            if (runner.next.data == erraseVal){

                let errased = runner.next;
                runner.next = runner.next.next;

                this.size--;
                return errased;
            }
            runner = runner.next;
        }
        return null;
    }

    front(){
        return this.head;
    }

    contain(searchVal){
        let runner = this.head;
        while(runner){
            if (runner.data == searchVal){
                return true;
            }
            runner = runner.next;
        }
        return false;
    }

    seed(arrayVals){
        if (arrayVals == null){ return "you fucked up!";}
        for (let i = 0; i < arrayVals.length; i++){
            this.enqueue(arrayVals[i]);
        }
        return;
    }

    display(){
        let runner = this.head;
        let queue = "";

        while(runner){
            if (runner.next){
                queue += ""+ runner.data+ " -> ";
            } else{
                queue += ""+ runner.data+ " _/";
            }
            runner=runner.next;
        }
        console.log(queue);
        return;
    }

    compareQueues(q2){
        if (this.size != q2.size){
            return false;
        } else if (this.isEmpty()){
            return true;
        }

        let runner1 = this.head;
        let runner2 = q2.head;

        while (runner1 && runner2){
            if (runner1.data != runner2.data){
                return false;
            }
            runner1 = runner1.next;
            runner2 = runner2.next;
        }
        return true;
    }

    isPalindrome(){
        let runner1 = this.head;
        let stackX = new Stack();

        while (runner1){
            stackX.push(runner1.data);
            runner1 = runner1.next;
        }

        runner1 = this.head;
        let runner2 = stackX.top;
        while(runner1){
            if (runner1.data != runner2.data){
                return false;
            }
            runner1 = runner1.next;
            runner2 = runner2.next;
        }
        return true;
    }

    findIntersection(q2){
    let runner1 = this.head;
    let stack1 = new Stack();

    let runner2 = q2.head;
    let stack2 = new Stack();

    let returnNode = null;

    while (runner1){
        stack1.push(runner1);
        runner1 = runner1.next;
    }
    while (runner2){
        stack2.push(runner2);
        runner2 = runner2.next;
    }

    runner1 = stack1.top;
    runner2 = stack2.top;
    while (runner1 && runner2){
        if (runner1.data === runner2.data){
            returnNode = runner1.data;
        } else{
            return returnNode;
        }
        runner1 = runner1.next;
        runner2 = runner2.next;
    }
    return returnNode;
}
}

var q2 = new Queue();