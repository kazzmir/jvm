public class Main {
    public static void main(String... args) {
        Object lock = new Main();
        synchronized (lock) { // monitorenter
            System.out.println("locked");
        } // monitorexit

        synchronized (lock) {
            System.out.println("locked again");
        }
        System.out.println("done");
    }
}
