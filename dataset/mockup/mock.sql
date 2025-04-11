

-- 👤 Sample users
INSERT INTO users (user_id, username, email, full_name, password_hash)
VALUES
    ('00000000-0000-0000-0000-000000000001', 'admin', 'admin@example.com', 'Admin User', 'hashedpassword1'),
    ('00000000-0000-0000-0000-000000000002', 'teller1', 'teller1@example.com', 'Teller One', 'hashedpassword2');

-- 🤖 Sample API Clients
INSERT INTO api_clients (client_id, client_name, client_secret_hash)
VALUES
    ('11111111-0000-0000-0000-000000000001', 'core-api-service', 'hashedsecret1');

-- 🧑‍⚖️ Roles
INSERT INTO roles (name, description)
VALUES
    ('admin', 'Administrator with full access'),
    ('teller', 'Bank teller with limited access'),
    ('auditor', 'Read-only access for auditing'),
    ('api_consumer', 'External system consuming API');

-- 🔗 user_roles
INSERT INTO user_roles (user_id, role_id)
SELECT '00000000-0000-0000-0000-000000000001', role_id FROM roles WHERE name = 'admin';

INSERT INTO user_roles (user_id, role_id)
SELECT '00000000-0000-0000-0000-000000000002', role_id FROM roles WHERE name = 'teller';

-- 🔗 api_client_roles
INSERT INTO api_client_roles (client_id, role_id)
SELECT '11111111-0000-0000-0000-000000000001', role_id FROM roles WHERE name = 'api_consumer';

-- 🎯 permissions
INSERT INTO permissions (name, description)
VALUES
    ('account.read', 'Read account information'),
    ('account.create', 'Create a new account'),
    ('transaction.create', 'Create a transaction'),
    ('transaction.read', 'Read transactions');

-- 🔗 role_permissions
INSERT INTO role_permissions (role_id, permission_id)
SELECT r.role_id, p.permission_id
FROM roles r, permissions p
WHERE
    (r.name = 'admin') OR
    (r.name = 'teller' AND p.name IN ('account.read', 'transaction.create')) OR
    (r.name = 'auditor' AND p.name IN ('account.read', 'transaction.read')) OR
    (r.name = 'api_consumer' AND p.name = 'account.read');

-- 🧑‍💼 Sample customers
INSERT INTO customers (customer_id, full_name, email, phone)
VALUES
    ('22222222-0000-0000-0000-000000000001', 'John Doe', 'john@example.com', '0812345678'),
    ('22222222-0000-0000-0000-000000000002', 'Jane Smith', 'jane@example.com', '0898765432');

-- 🏦 account_types
INSERT INTO account_types (name, interest_rate, minimum_balance)
VALUES
    ('Savings', 1.25, 500.00),
    ('Current', 0.00, 1000.00),
    ('Fixed Deposit', 2.50, 10000.00);

-- 💳 accounts
INSERT INTO accounts (account_id, customer_id, account_type_id, account_number, balance)
VALUES
    ('33333333-0000-0000-0000-000000000001', '22222222-0000-0000-0000-000000000001', 1, '001234567890', 1000.00),
    ('33333333-0000-0000-0000-000000000002', '22222222-0000-0000-0000-000000000002', 2, '009876543210', 2500.00);

-- 📈 interest_postings
INSERT INTO interest_postings (posting_id, account_id, period_start, period_end, interest_amount)
VALUES
    ('44444444-0000-0000-0000-000000000001', '33333333-0000-0000-0000-000000000001', '2025-01-01', '2025-03-31', 12.50);

