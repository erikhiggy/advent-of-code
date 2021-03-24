package aoc.days.day11;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class Day11 {
    public List<String> occupySeats(List<String> seats) {
        List<String> newSeats = new ArrayList<>(seats.size());
        String res = "";
        for (int row = 0; row<seats.size(); row++) {
            for (int i = 0; i<seats.get(0).length(); ++i) {
                // if we are at an unoccupied seat,
                // and no 4 other adjacent seats are occupied
                // occupy the seat

                if (seats.get(row).charAt(i) == 'L') {
                    // check for first row
                    if (row == 0) {
                        if (i == 0) { // beginning of the row
                            if (seats.get(row).charAt(i+1) != '#'
                                    && seats.get(row+1).charAt(i) != '#'
                                    && seats.get(row+1).charAt(i+1) != '#') {
                                res += '#';
                            }
                        } else if (i == seats.get(0).length()-1) { // end of the row
                            if (seats.get(row).charAt(i-1) != '#'
                                    && seats.get(row+1).charAt(i) != '#'
                                    && seats.get(row+1).charAt(i-1) != '#') {
                                res += '#';
                            }
                        } else { // any seat in between
                            if (seats.get(row).charAt(i-1) != '#'
                                    && seats.get(row).charAt(i+1) != '#'
                                    && seats.get(row+1).charAt(i) != '#'
                                    && seats.get(row+1).charAt(i-1) != '#'
                                    && seats.get(row+1).charAt(i+1) != '#') {
                                res += '#';
                            }
                        }
                    }
                    // check for last row
                    else if (row == seats.size()-1) {

                    }
                }
            }
        }
    }
}
