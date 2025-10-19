-- Seed users table with sample data
INSERT INTO users (id, name, email, created_at) VALUES 
('1', 'Alice Johnson', 'alice@example.com', datetime('now')),
('2', 'Bob Smith', 'bob@example.com', datetime('now')),
('3', 'Charlie Brown', 'charlie@example.com', datetime('now'))
ON CONFLICT(email) DO NOTHING;