public class Main{
    private int x;

    public Main(int a){
        this.x = a * 3;
    }

    public int get(int a){
        return a + this.x;
    }

    public static void main(String... args){
        Main m = new Main(5);
        System.out.println(m.get(4));
    }
}
