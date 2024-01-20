-- Create the ENUM type if it doesn't exist
CREATE TYPE status_enum AS ENUM ('undone', 'done');


-- Create the TABLE if it doesn't exist

    CREATE TABLE tasks (
        id SERIAL PRIMARY KEY,
        title VARCHAR(255) NOT NULL,
        status status_enum NOT NULL DEFAULT 'undone',  
        created_at TIMESTAMPTZ DEFAULT current_timestamp,
        updated_at TIMESTAMPTZ DEFAULT current_timestamp
    );
