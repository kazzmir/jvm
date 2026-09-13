public class Main{
    public static class Exception1 extends Exception {
    }

    public static class Exception2 extends Exception {
    }

    public static void foo() throws Exception {
        throw new Exception2();
    }

    public static void test(){
        try {
            foo();
        } catch (Exception1 e){
            System.out.println("exception1");
        } catch (Exception2 e){
            System.out.println("exception2");
        } catch (Exception e){
            System.out.println("exception");
        }
    }

    public static void main(String... args){
        Main.test();
    }
}
