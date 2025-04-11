-- DROP TABLES (if exist)
DROP TABLE IF EXISTS interest_postings CASCADE;
DROP TABLE IF EXISTS transactions CASCADE;
DROP TABLE IF EXISTS accounts CASCADE;
DROP TABLE IF EXISTS account_types CASCADE;
DROP TABLE IF EXISTS customers CASCADE;

-- ========================================
-- customers (optional)
-- ========================================
CREATE TABLE customers (
                           customer_id UUID PRIMARY KEY,
                           full_name TEXT NOT NULL,
                           email TEXT,
                           phone TEXT,
                           source TEXT DEFAULT 'local',
                           created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================================
-- account_types
-- ========================================
CREATE TABLE account_types (
                               account_type_id SMALLSERIAL PRIMARY KEY,
                               name TEXT NOT NULL UNIQUE,
                               interest_rate NUMERIC(5, 2) NOT NULL DEFAULT 0.0,
                               minimum_balance NUMERIC(16, 2) DEFAULT 0.0
);

-- ========================================
-- accounts
-- ========================================
CREATE TABLE accounts (
                          account_id UUID PRIMARY KEY,
                          customer_id UUID NOT NULL,
                          account_type_id SMALLINT NOT NULL REFERENCES account_types(account_type_id),
                          account_number VARCHAR(20) UNIQUE NOT NULL,
                          balance NUMERIC(18, 2) NOT NULL DEFAULT 0.0,
                          status TEXT NOT NULL DEFAULT 'active',
                          opened_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                          closed_at TIMESTAMP
);

CREATE INDEX idx_accounts_customer_id ON accounts(customer_id);

-- ========================================
-- transactions (partitioned table - ไม่มี PK/UNIQUE)
-- ========================================
CREATE TABLE transactions (
                              transaction_id UUID,
                              account_id UUID NOT NULL REFERENCES accounts(account_id),
                              txn_type TEXT NOT NULL CHECK (txn_type IN ('deposit', 'withdraw', 'transfer_in', 'transfer_out', 'interest')),
                              amount NUMERIC(18, 2) NOT NULL,
                              balance_after NUMERIC(18, 2) NOT NULL,
                              description TEXT,
                              txn_time DATE NOT NULL DEFAULT CURRENT_DATE
) PARTITION BY RANGE (txn_time);

-- ✅ ยังไม่สร้าง partition ใด
-- ✅ ไม่มี PRIMARY KEY หรือ UNIQUE เพื่อหลีกเลี่ยง constraint error

-- ========================================
-- interest_postings (❌ ไม่มี FK ไปที่ transactions)
-- ========================================
CREATE TABLE interest_postings (
                                   posting_id UUID PRIMARY KEY,
                                   account_id UUID NOT NULL REFERENCES accounts(account_id),
                                   period_start DATE NOT NULL,
                                   period_end DATE NOT NULL,
                                   interest_amount NUMERIC(18, 2) NOT NULL,
                                   posted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                   transaction_id UUID -- ❌ no FK
);



-- ========================
-- 🔄 DROP TABLES (ลบเรียงลำดับ)
-- ========================
DROP TABLE IF EXISTS role_permissions CASCADE;
DROP TABLE IF EXISTS permissions CASCADE;
DROP TABLE IF EXISTS api_client_roles CASCADE;
DROP TABLE IF EXISTS user_roles CASCADE;
DROP TABLE IF EXISTS roles CASCADE;
DROP TABLE IF EXISTS api_clients CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- ========================
-- 👤 users
-- ========================
CREATE TABLE users (
                       user_id UUID PRIMARY KEY,
                       username TEXT UNIQUE NOT NULL,
                       email TEXT,
                       full_name TEXT,
                       password_hash TEXT, -- สำหรับ local auth
                       auth_provider TEXT DEFAULT 'external', -- 'local', 'azure_ad', 'keycloak'
                       is_active BOOLEAN DEFAULT TRUE,
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================
-- 🤖 api_clients
-- ========================
CREATE TABLE api_clients (
                             client_id UUID PRIMARY KEY,
                             client_name TEXT NOT NULL UNIQUE,
                             client_secret_hash TEXT NOT NULL,
                             is_active BOOLEAN DEFAULT TRUE,
                             created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- ========================
-- 🧑‍⚖️ roles
-- ========================
CREATE TABLE roles (
                       role_id SERIAL PRIMARY KEY,
                       name TEXT UNIQUE NOT NULL, -- เช่น admin, teller, auditor
                       description TEXT
);

-- ========================
-- 🔗 user_roles (mapping ผู้ใช้กับบทบาท)
-- ========================
CREATE TABLE user_roles (
                            user_id UUID REFERENCES users(user_id) ON DELETE CASCADE,
                            role_id INTEGER REFERENCES roles(role_id) ON DELETE CASCADE,
                            source TEXT DEFAULT 'internal', -- internal / external
                            assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                            PRIMARY KEY (user_id, role_id)
);

-- ========================
-- 🔗 api_client_roles (mapping client กับบทบาท)
-- ========================
CREATE TABLE api_client_roles (
                                  client_id UUID REFERENCES api_clients(client_id) ON DELETE CASCADE,
                                  role_id INTEGER REFERENCES roles(role_id) ON DELETE CASCADE,
                                  assigned_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                  PRIMARY KEY (client_id, role_id)
);

-- ========================
-- 🎯 permissions (สิทธิ์ระดับ action)
-- ========================
CREATE TABLE permissions (
                             permission_id SERIAL PRIMARY KEY,
                             name TEXT UNIQUE NOT NULL, -- เช่น account.read, transaction.create
                             description TEXT
);

-- ========================
-- 🔗 role_permissions (mapping role กับสิทธิ์)
-- ========================
CREATE TABLE role_permissions (
                                  role_id INTEGER REFERENCES roles(role_id) ON DELETE CASCADE,
                                  permission_id INTEGER REFERENCES permissions(permission_id) ON DELETE CASCADE,
                                  PRIMARY KEY (role_id, permission_id)
);