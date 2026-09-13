public class Main{
    private int x;

    public Main(int x){
        this.x = x;
    }
    
    public int foo(int y){
        return x + y;
    }

    public static void test(){
        Main m = new Main(3);
        System.out.println(m.foo(3));
    }

    public static void main(String... args){
        Main.test();
    }
}
