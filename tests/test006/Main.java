public class Main{
    public int get(int x){
        switch (x){
            case 1: return 10;
            case 2: return 20;
            case 3: return 30;
        }

        return 0;
    }

    public static void main(String... args){
        System.out.println(new Main().get(2));
    }
}
