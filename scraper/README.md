# Rewards Maximizer Scraper

## Overview

The Rewards Maximizer Scraper is a Python-based web scraper designed to extract reward data from various reward program pages. It retrieves information about available rewards and stores it in a SQLite database for further analysis and user recommendations.

## Features

- Scrapes reward descriptions from:
  - Chick-fil-A
  - Navy Federal
  - McDonald's
  - TBD (Dummy Website | Hotel Rewards)
- Stores extracted reward data in a SQLite database.
- Utilizes **Beautiful Soup** for HTML parsing.
- Implements custom headers to simulate web browser behavior and avoid access restrictions.

## Requirements

- **Python 3.x**
- Required libraries:
  - `requests`
  - `beautifulsoup4`
  - `sqlite3`

## How to Run

To run the scraper, use the following command:

**_ex. run command:_** `poetry run python3 scraper.py`

Ensure that you have Poetry installed to manage dependencies effectively.

## Database Structure

The scraper creates and uses the following database tables:

- users
- companies
- rewards
- cards
- user_cards
- user_rewards
- user_feedback
- vendor_deals
- notifications
- comments

### Scraper Functions

The main functions within scraper.py include:

- scrape_chick_fil_a(): Retrieves reward data from Chick-fil-A's rewards page.
- scrape_navy_federal(): Extracts rewards information from Navy Federal's member deals page.
- scrape_mcdonalds(): Gathers rewards data from McDonald's rewards page.

Each scraping function:

- Sends a GET request to the respective URL.
- Parses the HTML response to extract reward details.
- Saves the extracted data into the SQLite database.

## Viewing Database Data

To view the data stored in the SQLite database, you can use the SQLite command line interface or any SQLite database viewer tool.

### Using SQLite Command Line Interface

1. **Open Terminal (or Command Prompt)** and navigate to the directory where your database file is located.

2. **Launch the SQLite shell** by executing the following command:

   **_ex. run command:_** `sqlite3 rewards_maximizer.db`

3. List all tables in the database with the command:

**_ex. run command:_** `.tables`

4. To exit the SQLite shell, type:

   **_ex. run command:_** `.exit`

### Example Queries

Here are some example SQL queries you can use to explore the data:

1. Get all rewards:

   **_ex. run command:_** `SELECT * FROM rewards;`

2. Find companies with a specific name:

   **_ex. run command:_** `SELECT * FROM companies WHERE name LIKE '%Chick-fil-A%';`
