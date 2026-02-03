import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

public class BowlingGameTest {

    @Test
    void gutterGameScoresZero() {
        BowlingGame g = new BowlingGame();
        rollMany(g, 20, 0);
        assertEquals(0, g.score());
    }

    @Test
    void allOnesScoresTwenty() {
        BowlingGame g = new BowlingGame();
        rollMany(g, 20, 1);
        assertEquals(20, g.score());
    }

    @Test
    void oneSpareAddsNextRollBonus() {
        BowlingGame g = new BowlingGame();
        g.roll(5);
        g.roll(5);  // spare
        g.roll(3);  // bonus
        rollMany(g, 17, 0);
        assertEquals(16, g.score()); // (10+3) + 3
    }

    @Test
    void oneStrikeAddsNextTwoRollsBonus() {
        BowlingGame g = new BowlingGame();
        g.roll(10); // strike
        g.roll(3);
        g.roll(4);
        rollMany(g, 16, 0);
        assertEquals(24, g.score()); // (10+3+4) + (3+4)
    }

    @Test
    void perfectGameScores300() {
        BowlingGame g = new BowlingGame();
        rollMany(g, 12, 10);
        assertEquals(300, g.score());
    }

    @Test
    void tenthFrameSpareGetsOneExtraRoll() {
        BowlingGame g = new BowlingGame();
        rollMany(g, 18, 0); // frames 1-9
        g.roll(7);
        g.roll(3); // spare in 10th
        g.roll(5); // bonus roll
        assertEquals(15, g.score());
    }

    @Test
    void tenthFrameStrikeGetsTwoExtraRolls() {
        BowlingGame g = new BowlingGame();
        rollMany(g, 18, 0); // frames 1-9
        g.roll(10); // strike in 10th
        g.roll(7);
        g.roll(2);
        assertEquals(19, g.score());
    }

    private static void rollMany(BowlingGame g, int times, int pins) {
        for (int i = 0; i < times; i++) {
            g.roll(pins);
        }
    }
}