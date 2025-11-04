-- Add up migration script here
alter table events
    add status enum ('pending', 'processing', 'success') default 'pending' not null;