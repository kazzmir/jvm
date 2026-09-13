public class Main{
    public static void main(String... args){
        // generate_class.py emits equivalent bytecode with explicit ldc variants.
        System.out.println("hi"); // ldc
        System.out.println("hi"); // ldc_w in the generated class
        System.out.println(42L); // ldc2_w
        System.out.println(2.5); // ldc2_w
    }
}
