public class Main{
    public static void test(int N){
        double[] data = new double[N];

        for (int i = 0; i < data.length; i++){
            data[i] = (double)(i * 2);
        }

        double total = 0;
        for (int i = 0; i < data.length; i++){
            total += data[i];
        }

        System.out.println(total);
    }

    public static void main(String... args){
        Main.test(10);
    }
}
