# Memory Lane Travel Journal

## Overview

Memory Lane Travel Journal is a travel enthusiast's companion, combining travel journaling with a historical twist. Users can document their travel experiences and explore historical events that occurred at their travel destinations, providing a unique historical context to their adventures.

## Prerequisites

Before you begin, ensure you have the following installed:

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Internet Computer SDK: [IC SDK Installation Guide](https://sdk.dfinity.org/docs/quickstart/local-quickstart.html)

## Getting Started

1. Clone the repository:

    ```bash
    git clone https://github.com/your-username/memory-lane-travel-journal.git
    cd memory-lane-travel-journal
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. Deploy the canister:

    ```bash
    npm run gen-deploy
    ```

## Project Structure

The project is structured as follows:

- `src/lib.rs`: Additional Rust modules and helper functions.

## Functions Hierarchy

1. **`add_travel_experience`**
    - Adds a new travel experience to the journal.

2. **`get_travel_experience`**
    - Retrieves details of a specific travel experience.

3. **`update_travel_experience`**
    - Updates details of an existing travel experience.

4. **`delete_travel_experience`**
    - Deletes a travel experience from the journal.

5. **`get_travel_experiences_before_date`**
    - Retrieves a list of travel experiences before a specified date.

6. **`update_travel_experience_date`**
    - Updates the date of a travel experience.

7. **`get_all_travel_experiences`**
    - Retrieves details of all travel experiences in the journal.

8. **`get_total_travel_experiences`**
    - Retrieves the total number of travel experiences.

9. **`get_travel_experiences_count_before_date`**
    - Retrieves the count of travel experiences before a specified date.

**Note:** This README assumes you have a basic understanding of Rust and the Internet Computer development environment.

Feel free to modify the README to match the specifics of your project or add more sections as needed.
