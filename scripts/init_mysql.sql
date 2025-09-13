CREATE TABLE IF NOT EXISTS admins (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  email VARCHAR(191) UNIQUE NOT NULL,
  password_hash VARCHAR(191) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS polls (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  starts_at DATETIME NOT NULL,
  ends_at DATETIME NOT NULL,
  status ENUM('draft','published','closed') NOT NULL DEFAULT 'draft',
  created_by BIGINT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  INDEX (status), INDEX (starts_at), INDEX (ends_at)
);

CREATE TABLE IF NOT EXISTS poll_options (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  poll_id BIGINT NOT NULL,
  label VARCHAR(255) NOT NULL,
  FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE,
  INDEX (poll_id)
);

-- votes_idx (hasil indexer on-chain → off-chain)
CREATE TABLE IF NOT EXISTS votes_idx (
  id BIGINT PRIMARY KEY AUTO_INCREMENT,
  poll_id BIGINT NOT NULL,
  option_id BIGINT NOT NULL,
  voter_addr VARCHAR(128) NOT NULL,
  tx_hash VARCHAR(128) NOT NULL,
  block_time DATETIME NOT NULL,
  UNIQUE KEY uniq_vote (poll_id, voter_addr),       -- 1 wallet 1 suara per poll (opsional)
  INDEX (poll_id), INDEX (option_id),
  FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE,
  FOREIGN KEY (option_id) REFERENCES poll_options(id) ON DELETE CASCADE
);

-- results_cache (agregat untuk dashboard cepat)
CREATE TABLE IF NOT EXISTS results_cache (
  poll_id BIGINT PRIMARY KEY,
  json_summary JSON NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  FOREIGN KEY (poll_id) REFERENCES polls(id) ON DELETE CASCADE
);
