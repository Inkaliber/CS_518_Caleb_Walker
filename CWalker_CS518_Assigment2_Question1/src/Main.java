import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        BowlingGame game = new BowlingGame();
        int myRolls = 0;

        System.out.println("Enter rolls as integers separated by spaces or new lines (e.g., '10 3 4 ...')");
        System.out.println("Enter a random character to end the game early. Otherwise, the game will only count up to 21 rolls.");
        System.out.println("Tip: For a full game you typically enter 20 rolls, or 21 with a 10th-frame bonus, or 12 strikes.");

        try (Scanner sc = new Scanner(System.in)) {
            while (sc.hasNextInt() && myRolls < 20) {
                int pins = sc.nextInt();
                game.roll(pins);
                myRolls += 1;
            }
        }

        System.out.println("Total score: " + game.score());
    }
}