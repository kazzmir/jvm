public class Main {
    public static void main(String... args) {
        byte[] data = new byte[4];
        data[0] = 42;
        data[1] = -128;
        data[2] = 127;

        for (int i = 0; i < data.length; i++) {
            System.out.println(data[i]);
        }
    }
}
