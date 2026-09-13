public class Main {
    public static void main(String... args) {
        float one = 1.0f;
        float two = one + one;

        System.out.println((double) one); // f2d
        System.out.println((int) one); // f2i
        System.out.println((float) one); // No-op: the JVM has no f2f instruction.

        System.out.println(one + two); // fadd
        System.out.println(one / two); // fdiv
        System.out.println(two * two); // fmul
        System.out.println(-two); // fneg
        System.out.println(one % two); // frem
        System.out.println(one - two); // fsub
    }
}
