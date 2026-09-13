public class Main{
    public static class Point{
        public Point(int p){
            this.p = p;
        }

        int p;
    }

    public static void test(int N){
        Point[] data = new Point[N];

        for (int i = 0; i < data.length; i++){
            data[i] = new Point(i * 2);
        }

        int total = 0;
        for (int i = 0; i < data.length; i++){
            total += data[i].p;
        }

        System.out.println(total);
    }

    public static void main(String... args){
        Main.test(10);
    }
}
