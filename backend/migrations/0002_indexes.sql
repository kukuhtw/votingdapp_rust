-- 0002_indexes.sql — buat index dengan guard agar idempotent

-- idx_polls_status_end
SET @exists := (
  SELECT COUNT(*) FROM information_schema.statistics
  WHERE table_schema = DATABASE() AND table_name = 'polls'
    AND index_name = 'idx_polls_status_end'
);
SET @sql := IF(@exists = 0,
  'CREATE INDEX `idx_polls_status_end` ON `polls` (`status`,`end`)',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- FULLTEXT ftx_polls_title_desc
SET @exists := (
  SELECT COUNT(*) FROM information_schema.statistics
  WHERE table_schema = DATABASE() AND table_name = 'polls'
    AND index_name = 'ftx_polls_title_desc'
);
SET @sql := IF(@exists = 0,
  'CREATE FULLTEXT INDEX `ftx_polls_title_desc` ON `polls` (`title`,`description`)',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- idx_votes_poll
SET @exists := (
  SELECT COUNT(*) FROM information_schema.statistics
  WHERE table_schema = DATABASE() AND table_name = 'votes_idx'
    AND index_name = 'idx_votes_poll'
);
SET @sql := IF(@exists = 0,
  'CREATE INDEX `idx_votes_poll` ON `votes_idx` (`poll_id`)',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- uq_votes_voter_poll
SET @exists := (
  SELECT COUNT(*) FROM information_schema.statistics
  WHERE table_schema = DATABASE() AND table_name = 'votes_idx'
    AND index_name = 'uq_votes_voter_poll'
);
SET @sql := IF(@exists = 0,
  'CREATE UNIQUE INDEX `uq_votes_voter_poll` ON `votes_idx` (`voter`,`poll_id`)',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;
