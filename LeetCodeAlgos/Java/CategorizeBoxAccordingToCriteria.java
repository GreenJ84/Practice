// Given four integers length, width, height, and mass, representing the dimensions and mass of a box, respectively, return a string representing the category of the box.
  // The box is "Bulky" if:
    // Any of the dimensions of the box is greater or equal to 104.
    // Or, the volume of the box is greater or equal to 109.
  // If the mass of the box is greater or equal to 100, it is "Heavy".
  // If the box is both "Bulky" and "Heavy", then its category is "Both".
  // If the box is neither "Bulky" nor "Heavy", then its category is "Neither".
  // If the box is "Bulky" but not "Heavy", then its category is "Bulky".
  // If the box is "Heavy" but not "Bulky", then its category is "Heavy".
// Note that the volume of the box is the product of its length, width and height.

public class CategorizeBoxAccordingToCriteria {
  public static void main(String[] args) {
    CategorizeBoxAccordingToCriteria obj = new CategorizeBoxAccordingToCriteria();

    testCategorizeBoxAccordingToCriteria(1, 1000, 35, 700, 300, "Heavy", obj);
    testCategorizeBoxAccordingToCriteria(2, 200, 50, 800, 50, "Neither", obj);
    testCategorizeBoxAccordingToCriteria(3, 2909, 3968, 3272, 727, "Both", obj);
  }

  private static void testCategorizeBoxAccordingToCriteria(int testNum, int length, int width, int height, int mass, String expected,CategorizeBoxAccordingToCriteria obj) {
    String result = obj.categorizeBox(length, width, height, mass);

    System.out.println(String.format(

      "Test %d: %s / %s (%s)",
      testNum,
      result,
      expected,
      result.equals(expected) ? "PASS" : "FAIL"
    ));
  }
  
  public String categorizeBox(int length, int width, int height, int mass) {
    boolean bulky = length >= 10000 ||
      width >= 10000 ||
      height >= 10000 ||
      (long) length * (long) width * (long) height >= 1000000000;
    boolean heavy = mass >= 100;
    return bulky ^ heavy ?
      bulky ? "Bulky" : "Heavy" :
      bulky ? "Both": "Neither";
  }
}