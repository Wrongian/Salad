CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    bio VARCHAR,
    is_private BOOLEAN NOT NULL,
    salt VARCHAR NOT NULL,
    display_name VARCHAR NOT NULL
);


CREATE TABLE IF NOT EXISTS links (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    next_id INT UNIQUE,
    description VARCHAR,
    title VARCHAR,
    href VARCHAR(255) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS images (
    id SERIAL PRIMARY KEY,
    img_src VARCHAR NOT NULL,
    filename VARCHAR NOT NULL,
    user_id INT,
    link_id INT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (link_id) REFERENCES links(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS follows (
    id SERIAL PRIMARY KEY,
    from_id INT NOT NULL,
    to_id INT NOT NULL,
    FOREIGN KEY (from_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (to_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS pending_follow_requests (
    id SERIAL PRIMARY KEY,
    from_id INT NOT NULL,
    to_id INT NOT NULL,
    FOREIGN KEY (from_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (to_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS reset_password_request (
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    code VARCHAR NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS notifications (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    trigger_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    notification_type INT NOT NULL,
    msg VARCHAR NOT NULL,
    is_read BOOLEAN NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (trigger_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE OR REPLACE FUNCTION reorder_link(node_id INT, new_position_id INT) RETURNS VOID AS $$
DECLARE
    current_next INT;
BEGIN
    IF node_id = new_position_id THEN 
        RETURN;
    END IF;
    SELECT next_id into current_next FROM links WHERE id = node_id; 
    UPDATE links SET next_id = NULL WHERE id = node_id;

    UPDATE links SET next_id = current_next WHERE next_id = node_id;

    IF new_position_id IS NULL THEN
        UPDATE links SET next_id = node_id WHERE next_id IS NULL AND id != node_id;
    ELSE
        UPDATE links SET next_id = node_id WHERE next_id = new_position_id;
        UPDATE links SET next_id = new_position_id WHERE id = node_id;
    END IF;

END;
$$ LANGUAGE plpgsql;

-- trigger to set newly inserted link as the new leaf node
CREATE OR REPLACE FUNCTION reorder_link_after_create() RETURNS TRIGGER AS $$
BEGIN
    IF NEW.next_id IS NULL THEN
        UPDATE links SET next_id = NEW.id WHERE links.next_id IS NULL AND links.id != NEW.id;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION reorder_link_after_delete() RETURNS TRIGGER AS $$ 
BEGIN 
    UPDATE links SET next_id = OLD.next_id WHERE links.next_id = OLD.id;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Subscribe the trigger to run on deletion in link table
CREATE TRIGGER reorder_links_trigger
AFTER DELETE on links
FOR EACH ROW
EXECUTE FUNCTION reorder_link_after_delete();

-- Subscribe the trigger to run on create in link table
CREATE TRIGGER reorder_links_on_create_trigger
AFTER INSERT on links
FOR EACH ROW
EXECUTE FUNCTION reorder_link_after_create();

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
