public class Main {
    public static void check(Object reference, int value) {
        // javac branches over each body when its condition is false.
        if (reference != null) { // ifnull
            System.out.println("non-null");
        }
        if (reference == null) { // ifnonnull
            System.out.println("null");
        }
        if (value != 0) { // ifeq
            System.out.println("nonzero");
        }
        if (value == 0) { // ifne
            System.out.println("zero");
        }
        if (value >= 0) { // iflt
            System.out.println("nonnegative");
        }
        if (value < 0) { // ifge
            System.out.println("negative");
        }
        if (value <= 0) { // ifgt
            System.out.println("nonpositive");
        }
        if (value > 0) { // ifle
            System.out.println("positive");
        }
    }

    public static void main(String... args) {
        // Exercise both taken and fall-through paths for every branch.
        check(null, -1);
        check(new Main(), 0);
        check(null, 1);
    }
}
