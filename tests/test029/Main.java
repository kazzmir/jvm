public class Main {
    private static int number;
    private static String message;

    public static void main(String... args) {
        number = 42; // putstatic
        message = "first"; // putstatic with a reference
        System.out.println(number);
        System.out.println(message);

        number = number + 1;
        message = "updated";
        System.out.println(number);
        System.out.println(message);
    }
}
