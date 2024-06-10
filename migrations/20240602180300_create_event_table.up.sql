CREATE TABLE events (
   id varchar(36) PRIMARY KEY ,
   partner_id SMALLINT NOT NULL ,
   timestamp TIMESTAMP NOT NULL ,
   comment TEXT,
   payload TEXT
);
