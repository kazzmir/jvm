public class Main {
    public static void printMessage(Object first, Object second, Object third,
                                   Object fourth, String message) {
        // Static parameters occupy slots 0..4, forcing indexed aload for slot 4.
        System.out.println(message); // aload 4
        String copy = message; // astore 5
        System.out.println(copy); // aload 5
    }

    public static void main(String... args) {
        printMessage(null, null, null, null, "indexed aload");
    }
}
