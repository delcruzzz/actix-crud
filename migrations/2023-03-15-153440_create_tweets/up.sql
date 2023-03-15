-- Your SQL goes here
create table tweet (
  id serial not null primary key,
  message varchar(140) not null,
  created_at timestamp not null,
)