import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import java.util.NoSuchElementException;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;

public class EightQueensCoroutine {

    // Like Python's queens(...) generator, but only expose queens(n).
    public static Iterable<List<Integer>> queens(int n) {
        return () -> new QueensIterator(n);
    }

    // Iterator that consumes solutions from a queue (producer thread = coroutine-like generator).
    private static final class QueensIterator implements Iterator<List<Integer>> {
        private static final Object END = new Object();

        private final BlockingQueue<Object> queue = new LinkedBlockingQueue<>();
        private Object nextItem; // either List<Integer> or END
        private boolean finished;

        QueensIterator(int n) {
            Thread producer = new Thread(() -> {
                try {
                    queensRec(n, 0, new ArrayList<>(), new ArrayList<>(), new ArrayList<>());
                } catch (InterruptedException ignored) {
                    Thread.currentThread().interrupt();
                } finally {
                    try {
                        queue.put(END);
                    } catch (InterruptedException e) {
                        Thread.currentThread().interrupt();
                    }
                }
            }, "nqueens-producer");

            producer.start();
            advance(); // prime first value (like starting a generator)
        }

        @Override
        public boolean hasNext() {
            return !finished;
        }

        @Override
        public List<Integer> next() {
            if (finished) throw new NoSuchElementException();
            @SuppressWarnings("unchecked")
            List<Integer> out = (List<Integer>) nextItem;
            advance();
            return out;
        }

        private void advance() {
            try {
                nextItem = queue.take();
                if (nextItem == END) finished = true;
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                finished = true;
            }
        }

        // Direct translation of the Python logic.
        private void queensRec(int n, int i, List<Integer> a, List<Integer> b, List<Integer> c)
                throws InterruptedException {

            if (i < n) {
                for (int j = 0; j < n; j++) {
                    int diag = i + j;
                    int anti = i - j;

                    if (!a.contains(j) && !b.contains(diag) && !c.contains(anti)) {
                        List<Integer> a2 = new ArrayList<>(a);
                        a2.add(j);

                        List<Integer> b2 = new ArrayList<>(b);
                        b2.add(diag);

                        List<Integer> c2 = new ArrayList<>(c);
                        c2.add(anti);

                        queensRec(n, i + 1, a2, b2, c2);
                    }
                }
            } else {
                // "yield a" -> queue solution
                queue.put(new ArrayList<>(a));
            }
        }
    }
}