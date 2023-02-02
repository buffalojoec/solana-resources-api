CREATE DATABASE resources;

CREATE TABLE resources.resources (
    id VARCHAR(45),
    resource_scope VARCHAR(255),
    resource_topic VARCHAR(255),
    resource_title VARCHAR(255),
    resource_type VARCHAR(255),
    resource_url VARCHAR(255),
);

INSERT INTO resources.resources (
    (
        "Solana",
        "Introduction to Solana Programming", 
        "Introduction",	
        "Blog",	
        "https://docs.solana.com/introduction"
    ),
    (
        "Solana",
        "Introduction to Solana Programming", 
        "An Introduction to Solana",	
        "Blog",	
        "https://grayscale.com/learn/an-introduction-to-solana/"
    ),
    (
        "Solana",
        "Introduction to Solana Programming", 
        "Solana Programming – The Ultimate Guide to Solana Development",	
        "Blog",	
        "https://moralis.io/solana-programming-the-ultimate-guide-to-solana-development/"
    ),
    (
        "Solana",
        "Introduction to Solana Programming", 
        "Programming on Solana - Introduction to Ethereum and Solana",	
        "Video",	
        "https://youtu.be/IQryb0J2QSU"
    )
);