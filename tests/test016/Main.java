public class Main {
    public static void main(String... args) {
        double one = 1.0;
        double two = one + one;

        System.out.println(one / two);
        System.out.println(two * two);
        System.out.println(-two);
        System.out.println(one % two);
        System.out.println(one - two);

        if (two > one) {
            System.out.println("greater");
        }
        if (one < two) {
            System.out.println("less");
        }
    }
}
