DROP INDEX `todo_title`;
CREATE INDEX `todo_title_done` ON `todo` (`title`, `done`);
