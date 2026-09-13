public class Main {
    public static void main(String... args) {
        int[][] data = new int[2][3]; // multianewarray [[I, 2
        data[0][1] = 42;
        data[1][2] = 7;

        System.out.println(data.length);
        System.out.println(data[0].length);
        System.out.println(data[0][1]);
        System.out.println(data[1][2]);
        System.out.println(data[1][1]); // Rows are independent and zero-initialized.
    }
}
