/**
 * Bowling Game kata implementation.
 * Assumes valid input rolls (typical kata constraint).
 */
public final class BowlingGame {

    // Max rolls: 21 (including 10th-frame bonus balls)
    private final int[] rolls = new int[21];
    private int currentRoll = 0;

    /** Record a roll with the number of pins knocked down. */
    public void roll(int pins) {
        rolls[currentRoll++] = pins;
    }

    /** Compute the total score for the game. */
    public int score() {
        int total = 0;
        int rollIndex = 0;

        for (int frame = 0; frame < 10; frame++) {
            if (isStrike(rollIndex)) {
                total += 10 + strikeBonus(rollIndex);
                rollIndex += 1;
            } else if (isSpare(rollIndex)) {
                total += 10 + spareBonus(rollIndex);
                rollIndex += 2;
            } else {
                total += frameSum(rollIndex);
                rollIndex += 2;
            }
        }

        return total;
    }

    private boolean isStrike(int rollIndex) {
        return rolls[rollIndex] == 10;
    }

    private boolean isSpare(int rollIndex) {
        return rolls[rollIndex] + rolls[rollIndex + 1] == 10;
    }

    private int strikeBonus(int rollIndex) {
        return rolls[rollIndex + 1] + rolls[rollIndex + 2];
    }

    private int spareBonus(int rollIndex) {
        return rolls[rollIndex + 2];
    }

    private int frameSum(int rollIndex) {
        return rolls[rollIndex] + rolls[rollIndex + 1];
    }
}