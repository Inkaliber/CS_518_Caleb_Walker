import java.util.List;

public class Main {
    public static void main(String[] args) {
        for (List<Integer> solution : EightQueensCoroutine.queens(8)) {
            System.out.println(solution); // prints like Python: [0, 4, 7, 5, 2, 6, 1, 3]
        }
    }
}