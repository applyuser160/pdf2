import pdf2
import os


def main() -> None:
    print("--- Testing PDF Generation ---")

    # 1. Create the native Rust objects directly.
    try:
        text1 = pdf2.TextBlock("This is a test.", 10.0, 280.0, 12.0)
        page1 = pdf2.Page(210.0, 297.0, [text1], [])
        doc_to_save = pdf2.Document([page1])
    except Exception as e:
        print(f"Error creating native objects: {e}")
        return

    output_filename = "test_final.pdf"

    # 2. Save the document to a PDF file by calling the native generate function.
    try:
        pdf2.generate(doc_to_save, output_filename)
        print(f"Successfully called generate function for '{output_filename}'")
    except Exception as e:
        print(f"Error during PDF generation: {e}")
        return

    # 3. Check if the file was created.
    if os.path.exists(output_filename):
        print(f"Success! Output file '{output_filename}' was created.")
    else:
        print(f"Error: Output file '{output_filename}' was not created.")


if __name__ == "__main__":
    main()
