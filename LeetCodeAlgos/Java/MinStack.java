import java.util.ArrayList;
import java.util.EmptyStackException;

public class MinStack {
  public static void main(String[] args) {
    testMinStack1();
    testMinStack2();
  }

  public static void testMinStack1(){
    MinStack minStack = new MinStack();
    minStack.push(-2);
    minStack.push(0);
    minStack.push(-3);
    assert minStack.getMin() == -3; // return -3
    minStack.pop();
    assert minStack.top() == 0; // return 0
    assert minStack.getMin() == -2; // return -2
  }

  public static void testMinStack2(){
    MinStack minStack = new MinStack();
    minStack.push(22);
    minStack.push(0);
    minStack.push(13);
    assert minStack.getMin() == 0; // return 0
    minStack.pop();
    assert minStack.top() == 0; // return 0
    assert minStack.getMin() == 0; // return 0
  }

  private ArrayList<Integer> stack;
  private int minIdx = -1;
  public MinStack() {
    this.stack = new ArrayList<>();
  }

  public void push(int val) {
    if (this.minIdx < 0) this.minIdx = 0;
    else if (this.stack.get(this.minIdx) > val) {
      this.minIdx = this.stack.size();
    }
    // Add to end
    stack.add(val);
  }

  public void pop() {
    // Remove last item
    stack.remove(stack.size() - 1);
    // Fix minIdx if no items
    if (this.stack.size() == 0) this.minIdx = -1;
    else if (this.minIdx >= this.stack.size()){
      this.minIdx = 0;
      for (int idx = 1; idx < this.stack.size(); idx++){
        if (this.stack.get(idx) < this.stack.get(minIdx)){
          this.minIdx = idx;
        }
      }
    }
  }

  public int top() {
    // Get top of stack -> last of Array
    return this.stack.get(this.stack.size() - 1);
  }

  public int getMin() {
    // Get the smallest current valuer in Stack
    if (this.minIdx < 0) throw new EmptyStackException();
    return this.stack.get(this.minIdx);
  }
}
