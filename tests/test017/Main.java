public class Main {
    private double value;

    public static void main(String... args) {
        Main object = new Main();
        double[] data = new double[1];

        System.out.println(object.value = 1.0);
        System.out.println(data[0] = 1.0);

        double copy;
        System.out.println(copy = 1.0); // dup2 before dstore_3
        System.out.println(copy);
    }
}
