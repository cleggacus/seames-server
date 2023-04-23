-- This file should undo anything in `up.sql`

drop table if exists users, blocks, documents, repositories, text_blocks, image_blocks;
drop type if exists tag;

