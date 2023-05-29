public class Main{
    public static int bar(int a){
        return a * 3;
    }

    public static int bar2(int a, int b, int c){
        return a * b + c;
    }

    public static void main(String... args){
        int x = 2;
        System.out.println(x + 2);
        System.out.println(x * 2);
        System.out.println(bar(4));
        System.out.println(bar2(3, 4, 5));
    }
}
