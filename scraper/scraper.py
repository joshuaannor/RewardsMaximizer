import requests
from bs4 import BeautifulSoup
from database import create_database, save_rewards_to_db
import time
from datetime import datetime

# Function to scrape rewards data from Chick-fil-A
def scrape_chick_fil_a():
    url = 'https://www.chick-fil-a.com/one'
    
    # Set custom headers
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, Gecko) Chrome/117.0.5938.62 Safari/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
        'Accept-Language': 'en-US,en;q=0.5',
        'Connection': 'keep-alive',
        'Upgrade-Insecure-Requests': '1',
        'Cache-Control': 'max-age=0',
    }
    
    session = requests.Session()
    response = session.get(url, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        print("Successfully retrieved Chick-fil-A page!")

        # Parse the page content
        soup = BeautifulSoup(response.text, 'html.parser')

        # Find the rewards items
        rewards_items = soup.select('div.blurb')

        # List to store the rewards data
        rewards_data = []
        seen_descriptions = set()  # To track duplicates

        # Get the current timestamp as a string
        current_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

        # Loop through the found items and extract the reward descriptions
        for item in rewards_items:
            reward_description = item.p.text.strip() if item.p else "No Description"
            reward_description = reward_description.lower()  # Standardize case

            # Check for duplicates
            if reward_description not in seen_descriptions:
                seen_descriptions.add(reward_description)
                rewards_data.append({
                    'company_id': 1,
                    'name': 'Chick-fil-A Reward',
                    'description': reward_description,
                    'timestamp': current_time  # Add timestamp
                })

        # Save rewards data to the database
        if rewards_data:
            print("------------------Chick-fil-A Rewards------------------")
            for data in rewards_data:
                print(f"Reward Description: {data['description']}, Timestamp: {data['timestamp']}")
            save_rewards_to_db(rewards_data)
        else:
            print("No Chick-fil-A rewards data found.")
    else:
        print(f"Failed to retrieve the Chick-fil-A page. Status code: {response.status_code}")

# Function to scrape rewards data from Navy Federal
def scrape_navy_federal():
    url = 'https://www.navyfederal.org/loans-cards/credit-cards/cardholder-resources/member-deals.html'

    # Set custom headers
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, Gecko) Chrome/117.0.5938.62 Safari/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
        'Accept-Language': 'en-US,en;q=0.5',
    }
    
    session = requests.Session()
    response = session.get(url, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        print("Successfully retrieved Navy Federal page!")

        # Parse the page content
        soup = BeautifulSoup(response.text, 'html.parser')

        # Find the rewards table
        rewards_table = soup.find('table', class_='table-resp')

        # List to store the rewards data
        rewards_data = []
        seen_cards = set()  # To track duplicates

        # Get the current timestamp as a string
        current_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

        if rewards_table:
            rows = rewards_table.find_all('tr')[1:]  # Skip header row
            for row in rows:
                columns = row.find_all('td')
                if columns:
                    credit_card = row.find('th').text.strip()  # Card name
                    without_deals = columns[0].text.strip()  # In-Store or Online Without Member Deals
                    with_deals = columns[1].text.strip()  # With Member Deals Purchase

                    # Create a unique identifier for the credit card
                    card_identifier = f"{credit_card.lower()} {without_deals} {with_deals}"

                    # Check for duplicates
                    if card_identifier not in seen_cards:
                        seen_cards.add(card_identifier)
                        rewards_data.append({
                            'company_id': 2,
                            'name': credit_card,
                            'description': f"Without Deals: {without_deals}, With Deals: {with_deals}",
                            'timestamp': current_time  # Add timestamp
                        })

        # Save rewards data to the database
        if rewards_data:
            print("------------------Navy Federal Rewards------------------")
            for data in rewards_data:
                print(f"Credit Card: {data['name']}, {data['description']}, Timestamp: {data['timestamp']}")
            save_rewards_to_db(rewards_data)
        else:
            print("No Navy Federal rewards data found.")
    else:
        print(f"Failed to retrieve the Navy Federal page. Status code: {response.status_code}")

# Function to scrape rewards data from McDonald's
def scrape_mcdonalds():
    url = "https://www.mcdonalds.com/ca/en-ca/getmoremcds/mymcdonaldsrewards.html" 

    # Set custom headers
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, Gecko) Chrome/117.0.5938.62 Safari/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
        'Accept-Language': 'en-US,en;q=0.5',
    }

    session = requests.Session()
    response = session.get(url, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        print("Successfully retrieved McDonald's page!")

        # Parse the page content
        soup = BeautifulSoup(response.text, 'html.parser')

        # Extract points and rewards information
        rewards = []
        seen_rewards = set()  # To track duplicates
        
        # Get the current timestamp as a string
        current_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

        for teaser in soup.find_all('div', class_='cmp-teaser'):
            title_element = teaser.find('h2')
            title = title_element.get_text(strip=True) if title_element else "No Title"

            # Try to find the description
            description_div = teaser.find('div', class_='cmp-teaser__description')
            description = description_div.get_text(strip=True) if description_div else "No Description"

            # Create a unique identifier for the reward
            reward_identifier = f"{title.lower()} {description.lower()}"

            # Check for duplicates
            if reward_identifier not in seen_rewards:
                seen_rewards.add(reward_identifier)
                # Check if title has points (e.g., "2,000 Points")
                if "Points" in title:
                    rewards.append({
                        'company_id': 3,
                        'name': title,
                        'description': description,
                        'timestamp': current_time  # Add timestamp
                    })

        # Save rewards data to the database
        if rewards:
            print("------------------McDonald's Rewards------------------")
            for reward in rewards:
                print(f"{reward['name']}: {reward['description']}, Timestamp: {reward['timestamp']}")
            save_rewards_to_db(rewards)
        else:
            print("No McDonald's rewards data found.")
    else:
        print(f"Failed to retrieve the McDonald's page. Status code: {response.status_code}")

# Function to scrape rewards data from Dummy Site
def scrape_dummy_site():
    url = "http://ens-labs.com/f24-Platinum-1/"

    # Set custom headers
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, Gecko) Chrome/117.0.5938.62 Safari/537.36',
        'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8',
        'Accept-Language': 'en-US,en;q=0.5',
    }

    session = requests.Session()
    response = session.get(url, headers=headers)

    # Check if the request was successful
    if response.status_code == 200:
        print("Successfully retrieved Dummy Site page!")

        # Parse the page content
        soup = BeautifulSoup(response.text, 'html.parser')

        # Extract credit card information
        rewards_data = []
        seen_cards = set()  # To track duplicates
        cards = soup.find_all('div', class_='card')

        # Get the current timestamp as a string
        current_time = datetime.now().strftime('%Y-%m-%d %H:%M:%S')

        for card in cards:
            title = card.h2.text.strip() if card.h2 else "No Title"
            
            # Find the next sibling and handle the case where it might not exist
            rewards_div = card.find_next_sibling('div', class_='rewards')
            if rewards_div:
                description = rewards_div.strong.text.strip() if rewards_div.strong else "No Description"
            else:
                description = "No Description"

            # Create a unique identifier for the credit card
            card_identifier = f"{title.lower()} {description.lower()}"

            # Check for duplicates
            if card_identifier not in seen_cards:
                seen_cards.add(card_identifier)
                rewards_data.append({
                    'company_id': 4,
                    'name': title,
                    'description': description,
                    'timestamp': current_time  # Add timestamp
                })

        # Save rewards data to the database
        if rewards_data:
            print("------------------Dummy Site Rewards------------------")
            for data in rewards_data:
                print(f"Credit Card: {data['name']}, Description: {data['description']}, Timestamp: {data['timestamp']}")
            save_rewards_to_db(rewards_data)
        else:
            print("No Dummy Site rewards data found.")
    else:
        print(f"Failed to retrieve the Dummy Site page. Status code: {response.status_code}")


if __name__ == "__main__":
    # Add a delay to avoid triggering blocks
    time.sleep(2)  # Delay for 2 seconds before scraping
    scrape_chick_fil_a()
    time.sleep(2)  # Additional delay before the next request
    scrape_navy_federal()
    time.sleep(2)  # Additional delay before the next request
    scrape_mcdonalds()
    time.sleep(2)  # Additional delay before the next request
    scrape_dummy_site()