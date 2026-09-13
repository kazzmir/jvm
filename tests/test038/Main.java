public class Main {
    private int value;

    public static void main(String... args) {
        Main object = new Main();
        int[] data = new int[1];

        System.out.println(object.value = 42); // dup_x1 before putfield
        System.out.println(object.value);
        System.out.println(data[0] = 7); // dup_x2 before iastore
        System.out.println(data[0]);
    }
}
