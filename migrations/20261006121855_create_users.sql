CREATE TABLE users (

    id SERIAL PRIMARY KEY,
    
    username VARCHAR(50) NOT NULL UNIQUE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
