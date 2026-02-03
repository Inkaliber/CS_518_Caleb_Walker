private static void printSolution(int[] sol) {
    int n = sol.length;
    for (int i : sol) {
        for (int c = 0; c < n; c++) {
            IO.print(i == c ? "Q " : ". ");
        }
        IO.println();
    }
    IO.println();
}

void main() {
    int n = 8; // Eight Queens

    EightQueens solver = new EightQueens(n);
    List<int[]> solutions = solver.solve();

    IO.println("Number of solutions: " + solutions.size());
    IO.println();

    for (int i = 0; i < solutions.size(); i++) {
        IO.println("Solution #" + (i + 1));
        printSolution(solutions.get(i));
    }
}