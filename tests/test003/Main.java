public class Main{
    public int get(int a){
        return a + 3;
    }

    public static void main(String... args){
        Main m = new Main();
        System.out.println(m.get(4));
    }
}
