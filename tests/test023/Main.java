public class Main {
    public static class Parent {
    }

    public static class Child extends Parent {
    }

    public static void check(Object value) {
        if (value instanceof Child) {
            System.out.println("Child");
        } else {
            System.out.println("not Child");
        }
        if (value instanceof Parent) {
            System.out.println("Parent");
        } else {
            System.out.println("not Parent");
        }
    }

    public static void main(String... args) {
        check(new Child());
        check(new Parent());
        check("unrelated");
        check(null);
    }
}
