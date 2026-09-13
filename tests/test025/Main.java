public class Main {
    public static void main(String... args) {
        int left = 12;
        int right = 10;
        int negative = -12;
        int shift = 2;

        System.out.println(left | right); // ior
        System.out.println(left & right); // iand
        System.out.println(negative >> shift); // ishr: sign extension
        System.out.println(left << shift); // ishl
        System.out.println(negative >>> shift); // iushr: zero extension
        System.out.println(left ^ right); // ixor

        // Int shifts use only the low five bits of the shift distance.
        shift = 34;
        System.out.println(negative >> shift);
        System.out.println(left << shift);
        System.out.println(negative >>> shift);
    }
}
