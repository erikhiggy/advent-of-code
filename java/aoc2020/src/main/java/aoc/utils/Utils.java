package aoc.utils;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

public class Utils {
    public List<String> readLines(String path) {
        List<String> lines = new ArrayList<>();
        try (BufferedReader bufferedReader = new BufferedReader(new FileReader(path))) {
            String line;
            while ((line = bufferedReader.readLine()) != null) {
                lines.add(line);
            }
        } catch (IOException ioException) {
            ioException.printStackTrace();
        }
        return lines;
    }

    public void printLines(List<String> lines) {
        for (String line : lines) {
            System.out.println(line);
        }
    }
}
