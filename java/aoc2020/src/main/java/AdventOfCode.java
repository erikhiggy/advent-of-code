import aoc.utils.Utils;

import java.util.List;

public class AdventOfCode {
    public static void main(String[] args) {
        Utils utils = new Utils();
        String path = "src/main/resources/day11/day11.txt";
        List<String> lines = utils.readLines(path);

        for (String line : lines) {
            for (int i = 0; i<line.length(); ++i) {
                char c = line.charAt(i);
                System.out.print(c);
            }
            System.out.println(" ");
        }
    }
}
