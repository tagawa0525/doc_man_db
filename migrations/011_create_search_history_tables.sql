-- migrations/011_create_search_history_tables.sql

-- 検索履歴テーブル
CREATE TABLE search_history (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    search_type TEXT NOT NULL, -- 'employee', 'business', 'document'
    search_query TEXT NOT NULL, -- JSON形式の検索条件
    result_count INTEGER,
    execution_time_ms INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

-- お気に入り検索テーブル
CREATE TABLE favorite_searches (
    id INTEGER PRIMARY KEY,
    employee_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    search_type TEXT NOT NULL,
    search_query TEXT NOT NULL, -- JSON形式の検索条件
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (employee_id) REFERENCES employees (id)
);

-- 検索候補テーブル
CREATE TABLE search_suggestions (
    id INTEGER PRIMARY KEY,
    search_type TEXT NOT NULL,
    field_name TEXT NOT NULL,
    suggestion TEXT NOT NULL,
    frequency INTEGER DEFAULT 1,
    last_used DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- インデックス作成
CREATE INDEX idx_search_history_employee ON search_history(employee_id, created_at);
CREATE INDEX idx_favorite_searches_employee ON favorite_searches(employee_id, is_active);
CREATE INDEX idx_search_suggestions_type ON search_suggestions(search_type, field_name);
CREATE INDEX idx_search_suggestions_frequency ON search_suggestions(frequency DESC, last_used DESC);