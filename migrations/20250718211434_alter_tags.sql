-- Add migration script here
ALTER TABLE tags ADD CONSTRAINT unique_board_color UNIQUE (board_uuid, color);
