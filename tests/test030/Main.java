public class Main {
    public static void main(String... args) {
        short[] data = new short[3];
        data[0] = 1000; // sipush, sastore
        data[1] = -1000; // sipush, sastore

        for (int i = 0; i < data.length; i++) {
            System.out.println(data[i]); // saload, sign-extended to int
        }
    }
}
