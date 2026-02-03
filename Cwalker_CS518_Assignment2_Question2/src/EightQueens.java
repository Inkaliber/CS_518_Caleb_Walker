import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class EightQueens {
    private final int n;

    // visited arrays
    private final boolean[] colUsed;    // columns
    private final boolean[] diagUsed;   // (row - col + n - 1)
    private final boolean[] antiUsed;   // (row + col)

    // board[row] = col
    private final int[] board;

    // list of solutions; each is an int[] of size n
    private final List<int[]> solutions = new ArrayList<>();

    public EightQueens(int n) {
        this.n = n;
        this.colUsed = new boolean[n];
        this.diagUsed = new boolean[2 * n - 1];
        this.antiUsed = new boolean[2 * n - 1];
        this.board = new int[n];
        Arrays.fill(this.board, -1);
    }

    public List<int[]> solve() {
        backtrack(0);
        return solutions;
    }

    private void backtrack(int row) {
        // If current row == n, a full placement is complete
        if (row == n) {
            solutions.add(board.clone());
            return;
        }

        // Try each column in this row
        for (int col = 0; col < n; col++) {
            int diag = row - col + (n - 1);
            int anti = row + col;

            // Skip if column/diagonal/anti-diagonal already used
            if (colUsed[col] || diagUsed[diag] || antiUsed[anti]) {
                continue;
            }

            // Place queen and mark visited
            board[row] = col;
            colUsed[col] = true;
            diagUsed[diag] = true;
            antiUsed[anti] = true;

            // Recurse to next row
            backtrack(row + 1);

            // Backtrack: remove queen and unmark visited
            board[row] = -1;
            colUsed[col] = false;
            diagUsed[diag] = false;
            antiUsed[anti] = false;
        }
    }
}