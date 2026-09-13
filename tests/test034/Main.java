public class Main {
    public static void check(int value) {
        // Sparse keys make javac emit lookupswitch rather than tableswitch.
        switch (value) {
            case -1000:
                System.out.println("negative");
                break;
            case 7:
                System.out.println("seven");
                break;
            case 1000:
                System.out.println("thousand");
                break;
            default:
                System.out.println("default");
        }
    }

    public static void main(String... args) {
        check(-1000);
        check(7);
        check(1000);
        check(-1001);
        check(0);
        check(1001);
    }
}
