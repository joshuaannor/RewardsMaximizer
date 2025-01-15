import sqlite3

def create_database():
    conn = sqlite3.connect('rewards_maximizer.db')
    cursor = conn.cursor()

    # Users table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS users (
        user_id INTEGER PRIMARY KEY AUTOINCREMENT,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL  -- Change to TEXT for string date
    )
    ''')

    # Companies table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS companies (
        company_id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        website TEXT NOT NULL,
        contact_email TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL  -- Change to TEXT for string date
    )
    ''')

    # Rewards table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS rewards (
        reward_id INTEGER PRIMARY KEY AUTOINCREMENT,
        company_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        description TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (company_id) REFERENCES companies(company_id) ON DELETE CASCADE
    )
    ''')

    # Cards table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS cards (
        card_id INTEGER PRIMARY KEY AUTOINCREMENT,
        company_id INTEGER NOT NULL,
        name TEXT NOT NULL,
        type TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (company_id) REFERENCES companies(company_id) ON DELETE CASCADE
    )
    ''')

    # User Cards table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS user_cards (
        user_card_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        card_id INTEGER NOT NULL,
        added TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        expires_on TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
        FOREIGN KEY (card_id) REFERENCES cards(card_id) ON DELETE CASCADE
    )
    ''')

    # User Rewards table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS user_rewards (
        user_reward_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        reward_id INTEGER NOT NULL,
        status TEXT NOT NULL,
        added TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        expires_on TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
        FOREIGN KEY (reward_id) REFERENCES rewards(reward_id) ON DELETE CASCADE
    )
    ''')

    # User Feedback table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS user_feedback (
        feedback_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        company_id INTEGER NOT NULL,
        rating INTEGER NOT NULL,
        comments TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
        FOREIGN KEY (company_id) REFERENCES companies(company_id) ON DELETE CASCADE
    )
    ''')

    # Vendor Deals table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS vendor_deals (
        deal_id INTEGER PRIMARY KEY AUTOINCREMENT,
        company_id INTEGER NOT NULL,
        title TEXT NOT NULL,
        description TEXT NOT NULL,
        valid_from DATE NOT NULL,
        valid_to DATE NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (company_id) REFERENCES companies(company_id) ON DELETE CASCADE
    )
    ''')

    # Notifications table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS notifications (
        notification_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        message TEXT NOT NULL,
        type TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    )
    ''')

    # Comments table
    cursor.execute('''
    CREATE TABLE IF NOT EXISTS comments (
        comment_id INTEGER PRIMARY KEY AUTOINCREMENT,
        user_id INTEGER NOT NULL,
        entity_type TEXT NOT NULL,
        comment_info TEXT NOT NULL,
        created TEXT NOT NULL,  -- Change to TEXT for string date
        updated TEXT NOT NULL,  -- Change to TEXT for string date
        FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    )
    ''')

    # Commit changes and close the connection
    conn.commit()
    conn.close()

def save_rewards_to_db(rewards_data):
    conn = sqlite3.connect('rewards_maximizer.db')
    cursor = conn.cursor()

    create_database() # I couldn't get it to work without this here
    # Save rewards data with current timestamp
    cursor.executemany('INSERT INTO rewards (company_id, name, description, created, updated) VALUES (?, ?, ?, ?, ?)', 
                       [(data['company_id'], data['name'], data['description'], data['timestamp'], data['timestamp']) for data in rewards_data])

    conn.commit()
    conn.close()