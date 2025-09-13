-- Tambah kolom pencatatan on-chain di tabel polls (aman berulang)
-- onchain_status: none|pending|success|failed
-- onchain_tx_hash: tx hash terakhir
-- onchain_at: unix seconds waktu push

-- onchain_status
-- 0003_onchain.sql
SET @exists := (
  SELECT COUNT(*) FROM information_schema.columns
  WHERE table_schema = DATABASE() AND table_name = 'polls' AND column_name = 'onchain_status'
);
SET @sql := IF(@exists=0,
  'ALTER TABLE `polls` ADD COLUMN `onchain_status` ENUM(''none'',''pending'',''success'',''failed'') NOT NULL DEFAULT ''none'' AFTER `status`',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- onchain_tx_hash
SET @exists := (
  SELECT COUNT(*) FROM information_schema.columns
  WHERE table_schema = DATABASE() AND table_name = 'polls' AND column_name = 'onchain_tx_hash'
);
SET @sql := IF(@exists=0,
  'ALTER TABLE `polls` ADD COLUMN `onchain_tx_hash` VARCHAR(128) NULL AFTER `onchain_status`',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- onchain_at
SET @exists := (
  SELECT COUNT(*) FROM information_schema.columns
  WHERE table_schema = DATABASE() AND table_name = 'polls' AND column_name = 'onchain_at'
);
SET @sql := IF(@exists=0,
  'ALTER TABLE `polls` ADD COLUMN `onchain_at` BIGINT NULL AFTER `onchain_tx_hash`',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;

-- index opsional
SET @exists := (
  SELECT COUNT(*) FROM information_schema.statistics
  WHERE table_schema = DATABASE() AND table_name = 'polls' AND index_name = 'idx_polls_onchain_status'
);
SET @sql := IF(@exists=0,
  'CREATE INDEX `idx_polls_onchain_status` ON `polls` (`onchain_status`)',
  'SELECT 1'
);
PREPARE s FROM @sql; EXECUTE s; DEALLOCATE PREPARE s;



