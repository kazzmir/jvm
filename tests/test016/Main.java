public class Main {
    // The two int parameters put the double in local slots 2 and 3.
    public static void testLocalSlot2(int first, int second) {
        double value = 1.0;
        System.out.println(value);
    }

    public static void main(String... args) {
        double one = 1.0; // dstore_1 / dload_1
        double two = one + one; // dstore_3 / dload_3

        System.out.println(one / two);
        System.out.println(two * two);
        System.out.println(-two);
        System.out.println(one % two);
        System.out.println(one - two);

        if (two > one) {
            System.out.println("greater");
        }
        if (one < two) {
            System.out.println("less");
        }

        testLocalSlot2(0, 0); // dstore_2 / dload_2 in the helper
    }
}
