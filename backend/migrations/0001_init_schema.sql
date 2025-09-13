-- =====================================================================
-- Schema: voting (MySQL 8+)
-- Catatan:  0001_init_schema.sql
-- - Semua identifier dibungkus backtick (`) agar aman (mis. kolom `end`)
-- - ENGINE/CHARSET diset eksplisit
-- - Tipe/kolom mengikuti rancangan kamu (id BINARY(16), waktu unix BIGINT)
-- =====================================================================

-- Admins
CREATE TABLE IF NOT EXISTS `admins` (
  `id`            BINARY(16)    NOT NULL,
  `email`         VARCHAR(191)  NOT NULL UNIQUE,
  `password_hash` VARCHAR(255)  NOT NULL,
  `created_at`    BIGINT        NOT NULL DEFAULT (UNIX_TIMESTAMP()),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- Polls
CREATE TABLE IF NOT EXISTS `polls` (
  `id`            BINARY(16)    NOT NULL,
  `slug`          VARCHAR(191)  NOT NULL UNIQUE,
  `title`         VARCHAR(255)  NOT NULL,
  `description`   TEXT          NOT NULL,
  `options_json`  TEXT          NOT NULL,           -- JSON array of options
  `start`         BIGINT        NULL,               -- unix seconds (nullable)
  `end`           BIGINT        NOT NULL,           -- unix seconds (expiry)
  `vote_price`    VARCHAR(64)   NOT NULL,           -- store as string (micro units)
  `status`        ENUM('draft','published','archived') NOT NULL DEFAULT 'draft',
  `created_at`    BIGINT        NOT NULL,
  `updated_at`    BIGINT        NOT NULL DEFAULT (UNIX_TIMESTAMP()),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- Votes index (off-chain analytics)
CREATE TABLE IF NOT EXISTS `votes_idx` (
  `tx_hash`      VARCHAR(128) NOT NULL,
  `poll_id`      VARCHAR(128) NOT NULL,
  `voter`        VARCHAR(128) NOT NULL,
  `option_index` INT          NOT NULL,
  `amount`       VARCHAR(64)  NOT NULL,
  `block_time`   BIGINT       NOT NULL,
  `chain_id`     VARCHAR(64)  NOT NULL,
  PRIMARY KEY (`tx_hash`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- Results cache (optional)
CREATE TABLE IF NOT EXISTS `results_cache` (
  `poll_id`     VARCHAR(128) NOT NULL PRIMARY KEY,
  `tallies`     TEXT         NOT NULL,  -- JSON array of strings
  `total`       VARCHAR(64)  NOT NULL,
  `updated_at`  BIGINT       NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

