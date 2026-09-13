public class Main {
    public interface ValueSource {
        int getValue();
    }

    public static class FixedValue implements ValueSource {
        public int getValue() {
            return 7;
        }
    }

    public static void main(String... args) {
        int value = 42;
        // Java 9+ compiles runtime string concatenation using invokedynamic.
        System.out.println("value=" + value);
        value += 1;
        System.out.println("next=" + value);

        ValueSource source = new FixedValue();
        System.out.println(source.getValue()); // invokeinterface
    }
}
