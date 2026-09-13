public class Main {
    public static long square(long value) {
        return value * value; // lmul, lreturn
    }

    public static void main(String... args) {
        long[] data = new long[2];
        long one = 1L; // lconst_1, lstore_2
        long two = one + one; // ladd, indexed lstore
        long zero = 0L; // lconst_0

        System.out.println((double) two); // lload, l2d
        System.out.println((float) two); // l2f
        System.out.println((int) two); // l2i
        System.out.println(one + two); // ladd
        System.out.println(one & two); // land

        long negative = zero - (one + two); // lsub: -3
        int shift = 1;
        System.out.println(negative % two); // lrem: -1
        System.out.println(one << shift); // lshl
        System.out.println(negative >> shift); // lshr: sign extension
        System.out.println(negative >>> shift); // lushr: zero extension
        System.out.println(one ^ two); // lxor
        System.out.println(negative); // lsub result

        System.out.println(negative / two); // ldiv: truncates toward zero
        System.out.println(square(two)); // lmul and lreturn in the helper
        System.out.println(-two); // lneg
        System.out.println(one | two); // lor

        // Long shifts use only the low six bits of the int shift distance.
        shift = 65;
        System.out.println(one << shift);
        System.out.println(negative >> shift);
        System.out.println(negative >>> shift);

        data[0] = two; // lastore
        System.out.println(data[0]); // laload
        System.out.println(data[1]); // zero-initialized long element

        if (two > one) { // lcmp
            System.out.println("greater");
        }
        if (zero < one) { // lcmp
            System.out.println("less");
        }
        if (data[1] == zero) { // lcmp
            System.out.println("equal");
        }
    }
}
