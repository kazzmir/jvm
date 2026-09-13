public class Main {
    public static void main(String... args) {
        int value = 42;
        // Java 9+ compiles runtime string concatenation using invokedynamic.
        System.out.println("value=" + value);
        value += 1;
        System.out.println("next=" + value);
    }
}
