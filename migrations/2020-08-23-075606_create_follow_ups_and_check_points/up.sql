-- Your SQL goes here
CREATE TABLE check_points (
    id serial PRIMARY KEY,
    comments text,
    manager_actions text,
    managee_actions Text,
    highlights Text,
    mood integer,
    goals Text,
    previous_actions_status Text,
    check_point_date TIMESTAMP
);

CREATE TABLE follow_ups (
   id serial PRIMARY KEY,
   managee_id serial REFERENCES employees(id),
   check_points_ids bigint[]
);
