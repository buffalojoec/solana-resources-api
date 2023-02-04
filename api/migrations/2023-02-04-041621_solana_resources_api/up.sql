CREATE TABLE IF NOT EXISTS resources (
    id              UUID PRIMARY KEY        NOT NULL,
    resource_scope  TEXT                    NOT NULL,
    resource_topic  TEXT                    NOT NULL,
    resource_title  TEXT                    NOT NULL,
    resource_type   TEXT                    NOT NULL,
    resource_url    TEXT                    NOT NULL
);

INSERT INTO resources (
    id, 
    resource_scope, 
    resource_topic, 
    resource_title, 
    resource_type, 
    resource_url
) VALUES 
    (
        '631b579d-ab49-48ac-b836-828d7c4b6670',
        'Solana',
        'Introduction to Solana Programming', 
        'Introduction',	
        'Blog',	
        'https://docs.solana.com/introduction'
    ),
    (
        'e2218eab-527f-4331-ae33-82ec5309edb8',
        'Solana',
        'Introduction to Solana Programming', 
        'An Introduction to Solana',	
        'Blog',	
        'https://grayscale.com/learn/an-introduction-to-solana/'
    ),
    (
        'ba2f85b8-9cb6-4f34-b664-595edd061c3f',
        'Solana',
        'Introduction to Solana Programming', 
        'Solana Programming – The Ultimate Guide to Solana Development',	
        'Blog',	
        'https://moralis.io/solana-programming-the-ultimate-guide-to-solana-development/'
    ),
    (
        'ccc9b7bb-a640-4b60-b093-3c4d78094690',
        'Solana',
        'Introduction to Solana Programming', 
        'Programming on Solana - Introduction to Ethereum and Solana',	
        'Video',	
        'https://youtu.be/IQryb0J2QSU'
    );