public class Main {
    public static void main(String... args) {
        int value = 127;
        value *= 127;
        value *= 3;

        System.out.println((byte) value); // i2b
        System.out.println((int) (char) value); // i2c, printed numerically
        System.out.println((double) value); // i2d
        System.out.println((float) value); // i2f
        System.out.println((long) value); // i2l
        System.out.println((short) value); // i2s
    }
}
