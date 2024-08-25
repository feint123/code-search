public class Example {

    private int number;
    private String text;

    public Example(int number, String text) {
        this.number = number;
        this.text = text;
    }

    public int getNumber() {
        return number;
    }

    public void setNumber(int number) {
        this.number = number;
    }

    public String getText() {
        return text;
    }

    public void setText(String text) {
        this.text = text;
    }

    public void printInfo() {
        System.out.println("Number: " + number + ", Text: " + text);
    }

    public static void main(String[] args) {
        Example example = new Example(42, "Hello, World!");
        example.printInfo();
    }
}

interface Printable {
    void print();
}

enum Color {
    RED,
    GREEN,
    BLUE,
}
