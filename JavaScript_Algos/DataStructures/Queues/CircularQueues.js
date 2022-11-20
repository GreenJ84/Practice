class QueueNode{
    constructor(data){
        this.data = data;
        this.next = null;
    }

}

class CircularQueue{
    constructor(){
        this.head = null;
        this.tail = null;
        this.size = 0;
        this.cap = 10;
    }

    isEmpty(){
        if (this.head == null){
            return true;
        }
        return false;
    }

    isFull(){
        if (this.size == this.cap){
            return true
        }
        return false
    }

    size(){
        return this.size
    }

    expandStorage( newSize ){
        if (newSize <= this.cap){
            return 'Use Reduce method for a smaller queue.'
        }
        this.cap = newSize
    }

    shrinkStorage( newSize ){
        if (newSize >= this.cap){
            return 'Use Expand method for a larger queue.'
        }
        this.cap = newSize
    }

    enqueue(val){
        let newNode = new QueueNode(val);

        if (this.head == null){
            this.head = newNode;
            this.tail = newNode;
            this.size++;
        } else if (!this.isFull()){
            this.tail.next = newNode;
            this.tail = this.tail.next;
            this.size++;
        } else{
            return 'Failed Operation. Expand Storage'
        }
        return this.size;
    }

    dequeue(){
        if (this.isEmpty()){
            return 'Failed. Queue Null';
        }
        let oldHead = this.head;
        this.head = this.head.next || null;
        this.size--;
        return oldHead;
    }

    dequeueVal(erraseVal){
        if (this.isEmpty()){
            return 'Failed. Queue Null';
        }
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
        return 'Failed. Value not in Queue';
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

}

var q2 = new CircularQueue();

