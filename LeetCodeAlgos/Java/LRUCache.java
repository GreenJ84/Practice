
import java.util.*;

public class LRUCache {
  Node head;
  Node tail;
  Map < Integer, Node > map;
  int capacity;

  public LRUCache(int capacity) {
      this.capacity = capacity;
      this.map = new HashMap<Integer, Node>();
      this.head = new Node(0, 0);
      this.tail = new Node(0, 0);

      head.next = tail;
      tail.prev = head;
  }

  public int get(int key) {
      if (this.map.containsKey(key)) {
          Node node = this.map.get(key);
          remove(node);
          insert(node);
          return node.value;
      } else {
          return -1;
      }
  }

  public void put(int key, int value) {
      if (this.map.containsKey(key)) {
          remove(this.map.get(key));
      }
      else if (this.map.size() == capacity) {
          remove(tail.prev);
      }
      insert(new Node(key, value));
  }

  private void remove(Node node) {
      this.map.remove(node.key);
      node.prev.next = node.next;
      node.next.prev = node.prev;
  }

  private void insert(Node node) {
      this.map.put(node.key, node);
      node.next = head.next;
      node.next.prev = node;
      head.next = node;
      node.prev = head;
  }

  class Node {
      Node prev, next;
      int key, value;
      Node(int key, int value) {
          this.key = key;
          this.value = value;
      }
  }
}