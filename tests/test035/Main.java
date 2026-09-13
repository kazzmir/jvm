public class Main {
    public static void checkReferences(Object left, Object right) {
        // javac branches over each body when the condition is false.
        if (left != right) { // if_acmpeq
            System.out.println("different references");
        }
        if (left == right) { // if_acmpne
            System.out.println("same reference");
        }
    }

    public static void checkIntegers(int left, int right) {
        if (left != right) { // if_icmpeq
            System.out.println("unequal");
        }
        if (left == right) { // if_icmpne
            System.out.println("equal");
        }
        if (left >= right) { // if_icmplt
            System.out.println("greater or equal");
        }
        if (left < right) { // if_icmpge
            System.out.println("less");
        }
        if (left <= right) { // if_icmpgt
            System.out.println("less or equal");
        }
        if (left > right) { // if_icmple
            System.out.println("greater");
        }
    }

    public static void main(String... args) {
        Object value = new Main();
        checkReferences(value, value);
        checkReferences(value, new Main());
        checkReferences(null, null);
        checkReferences(value, null);
        checkReferences(null, value);

        // Exercise both outcomes of every integer comparison.
        checkIntegers(-2, 3);
        checkIntegers(3, -2);
        checkIntegers(3, 3);
    }
}
