# litrev: Feature Roadmap

A command-line tool to streamline the academic literature review process.

## Core Features

* **Project Initialization & Management**
    * `litrev init`: Creates a new `.litrev/` directory in the current folder to track literature.

* **Automatic Literature Indexing**
    * **File Ingestion**: Automatically detects and parses `.bib` files to extract metadata (author, title, year, abstract, etc.).
    * **PDF Linking**: Intelligently links BibTeX entries to their corresponding PDF files based on the citation key or filename.
    * **Database**: Stores all metadata in a local, fast-querying database (e.g., SQLite) within the `.litrev/` directory.

* **Powerful Search Capability**
    * `litrev search <keywords>`: Searches across titles, authors, abstracts, and keywords from the indexed `.bib` files.
    * **Advanced Filtering**: `litrev search --author "Kenway" --year 2019`: Allows for targeted searches with flags.

* **Direct File Access**
    * `litrev open <citekey>`: Immediately opens the linked PDF for a given citation key in the system's default PDF viewer. This is a key feature for reducing friction.

* **Library Insights**
    * `litrev insights`: Analyzes the entire library to provide useful metrics:
        * **Top Authors**: Lists the most frequently occurring authors.
        * **Keyword Cloud**: Shows the most common keywords across all papers.
        * **Publication Timeline**: A simple graph showing the number of papers by year.

## Additional Features (Future Enhancements)

These features would build on the core functionality to make `litrev` an indispensable tool.

* **AI-Powered Summarization**
    * `litrev summary <citekey>`: Integrates with a large language model (LLM) to provide a concise summary of the paper's abstract or even the full text. This could be invaluable for quickly assessing complex papers, especially in fields like hypersonic flow where the theory is dense.

* **Citation & Reference Management**
    * **Cross-Referencing**: `litrev find-refs <citekey>`: Scans the full text of all PDFs to find where a specific paper is cited within your local library.
    * **Note-Taking**: `litrev note <citekey>`: Opens a markdown file linked to a specific entry for taking notes.

* **Advanced Organisation**
    * **Tagging System**: `litrev tag <citekey> --add "adjoint-methods, cfd"`: Allows for custom, searchable tags to be added to entries.
    * **Project Groups**: `litrev group "LSS-methods"`: Creates sub-collections of papers within the main library for specific projects or topics.

* **Usability & Integration**
    * **Shell Integration**: Provides shell completions (e.g., for `bash` or `zsh`) so you can tab-complete citation keys.
    * **Web Exporter**: `litrev export --html`: Generates a simple, static HTML site to view your library in a browser.
    * **Cloud Sync**: Option to sync the `.litrev/` database and notes via a service like Dropbox or Git for use across multiple machines.