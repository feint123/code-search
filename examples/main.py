class Author:
    def __init__(self, name, email):
        self.name = name
        self.email = email

    def __str__(self):
        return f"Author: {self.name}, Email: {self.email}"


class Book:
    def __init__(self, title, author, publication_year):
        self.title = title
        self.author = author
        self.publication_year = publication_year

    def __str__(self):
        return f"Title: {self.title}, Author: {self.author}, Year: {self.publication_year}"


class Library:
    def __init__(self):
        self.books = []
        self.authors = []

    def add_author(self, name, email):
        new_author = Author(name, email)
        self.authors.append(new_author)
        return new_author

    def add_book(self, title, author, year):
        if author not in self.authors:
            print(f"Author '{author}' is not registered.")
            return
        book = Book(title, author, year)
        self.books.append(book)

    def find_book_by_title(self, title):
        for book in self.books:
            if book.title == title:
                return book
        return None

    def list_books(self):
        for book in self.books:
            print(book)


def main():
    library = Library()

    # Adding authors
    author1 = library.add_author("John Doe", "johndoe@example.com")
    author2 = library.add_author("Jane Smith", "janesmith@example.com")

    # Adding books
    library.add_book("The Great Adventure", author1, 2020)
    library.add_book("Python for Beginners", author2, 2019)

    # Listing all books
    print("Books in the library:")
    library.list_books()

    # Finding a book by title
    found_book = library.find_book_by_title("Python for Beginners")
    if found_book:
        print("Found book:", found_book)
    else:
        print("Book not found.")


if __name__ == "__main__":
    main()
