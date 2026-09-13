public class Main {
    public static void main(String... args) {
        float zero = 0.0f; // fconst_0
        float one = 1.0f; // fconst_1
        float two = 2.0f; // fconst_2

        System.out.println((double) one); // f2d
        System.out.println((long) one); // f2l
        System.out.println((int) one); // f2i
        System.out.println((float) one); // No-op: the JVM has no f2f instruction.

        System.out.println(one + two); // fadd
        System.out.println(one / two); // fdiv
        System.out.println(two * two); // fmul
        System.out.println(-two); // fneg
        System.out.println(one % two); // frem
        System.out.println(one - two); // fsub

        if (two > one) { // fcmpl
            System.out.println("greater");
        }
        if (zero < one) { // fcmpg
            System.out.println("less");
        }
    }
}
