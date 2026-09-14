public class Main {
    public static class Data {
        int[] data;
        Data self;

        public Data(int id) {
            self = this; // Strong cycle: reference counting alone cannot reclaim it.
            data = new int[8192];
            data[0] = id;
            data[8191] = -id;
        }
    }

    public static void main(String... args) {
        int iterations = 100000;
        // Only 64 recent objects stay reachable: about 2 MiB of array payload.
        Data[] live = new Data[64];
        Data anchor = new Data(123);
        for (int i = 0; i < iterations; i++) {
            int slot = i % 64;
            if (i >= 64) {
                // Reachable objects must survive allocation pressure intact.
                if (live[slot].self != live[slot] || live[slot].self.data[0] != i - 64
                        || live[slot].data[8191] != -(i - 64)) {
                    System.out.println("FAIL: live object corrupted");
                    return;
                }
            }
            // Replacing this external reference leaves an unreachable self-cycle.
            // Its reference count stays nonzero, retaining the array too.
            // Total array allocation exceeds 3 GiB; never break the cycles.
            live[slot] = new Data(i);
        }
        for (int i = iterations - 64; i < iterations; i++) {
            if (live[i % 64].self != live[i % 64] || live[i % 64].self.data[0] != i
                    || live[i % 64].data[8191] != -i) {
                System.out.println("FAIL: retained object corrupted");
                return;
            }
        }
        if (anchor.self != anchor || anchor.self.data[0] != 123 || anchor.data[8191] != -123) {
            System.out.println("FAIL: anchor corrupted");
            return;
        }
        System.out.println("allocation stress passed");
    }
}
