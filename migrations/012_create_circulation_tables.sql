-- 回覧定義テーブル
CREATE TABLE circulation_workflows (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    steps TEXT NOT NULL,        -- JSON workflow steps
    is_active BOOLEAN DEFAULT 1,
    created_by INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES employees (id)
);

-- 回覧インスタンステーブル
CREATE TABLE document_circulations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    document_id INTEGER NOT NULL,
    workflow_id INTEGER NOT NULL,
    initiated_by INTEGER NOT NULL,
    current_step INTEGER DEFAULT 1,
    status TEXT DEFAULT 'active', -- 'active', 'completed', 'cancelled'
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    notes TEXT,
    FOREIGN KEY (document_id) REFERENCES documents (id),
    FOREIGN KEY (workflow_id) REFERENCES circulation_workflows (id),
    FOREIGN KEY (initiated_by) REFERENCES employees (id)
);

-- 回覧ステップテーブル
CREATE TABLE circulation_steps (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    circulation_id INTEGER NOT NULL,
    step_number INTEGER NOT NULL,
    assignee_id INTEGER NOT NULL,
    action_required TEXT NOT NULL, -- 'review', 'approve', 'acknowledge'
    status TEXT DEFAULT 'pending', -- 'pending', 'completed', 'skipped'
    assigned_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    completed_at DATETIME,
    comments TEXT,
    FOREIGN KEY (circulation_id) REFERENCES document_circulations (id),
    FOREIGN KEY (assignee_id) REFERENCES employees (id)
);

-- インデックス作成
CREATE INDEX idx_document_circulations_document_id ON document_circulations (document_id);
CREATE INDEX idx_document_circulations_status ON document_circulations (status);
CREATE INDEX idx_circulation_steps_circulation_id ON circulation_steps (circulation_id);
CREATE INDEX idx_circulation_steps_assignee_id ON circulation_steps (assignee_id);
CREATE INDEX idx_circulation_steps_status ON circulation_steps (status);

-- 基本的なワークフローを挿入
INSERT INTO circulation_workflows (name, description, steps, created_by) VALUES 
(
    '標準承認ワークフロー',
    '一般的な文書承認プロセス',
    '[
        {"step_number": 1, "assignee_role": "manager", "action_required": "review", "is_optional": false, "timeout_hours": 72},
        {"step_number": 2, "assignee_role": "director", "action_required": "approve", "is_optional": false, "timeout_hours": 48}
    ]',
    1
),
(
    '簡易確認ワークフロー',
    '簡単な確認のみのプロセス',
    '[
        {"step_number": 1, "assignee_role": "manager", "action_required": "acknowledge", "is_optional": false, "timeout_hours": 24}
    ]',
    1
),
(
    '高重要度承認ワークフロー',
    '重要文書の多段階承認プロセス',
    '[
        {"step_number": 1, "assignee_role": "manager", "action_required": "review", "is_optional": false, "timeout_hours": 48},
        {"step_number": 2, "assignee_role": "director", "action_required": "review", "is_optional": false, "timeout_hours": 48},
        {"step_number": 3, "assignee_role": "executive", "action_required": "approve", "is_optional": false, "timeout_hours": 72}
    ]',
    1
);