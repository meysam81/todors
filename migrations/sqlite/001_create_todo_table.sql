CREATE TABLE `todo` (
    `id` INTEGER PRIMARY KEY AUTOINCREMENT,
    `title` TEXT UNIQUE NOT NULl,
    `done` BOOLEAN NOT NULL DEFAULT 0,
    `description` TEXT,
    `created_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER `todo_updated_at` AFTER UPDATE ON `todo` FOR EACH ROW
BEGIN
    UPDATE `todo` SET `updated_at` = CURRENT_TIMESTAMP WHERE `id` = NEW.`id`;
END;

CREATE INDEX `todo_title` ON `todo` (`title`);
